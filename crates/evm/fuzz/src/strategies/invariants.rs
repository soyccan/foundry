use super::{fuzz_calldata_with_config, fuzz_param_from_state, CalldataFuzzDictionary};
use crate::{
    invariant::{BasicTxDetails, FuzzRunIdentifiedContracts, SenderFilters},
    strategies::{fuzz_calldata_from_state, fuzz_param, EvmFuzzState},
};
use alloy_json_abi::{Function, JsonAbi};
use alloy_primitives::{Address, Bytes};
use parking_lot::RwLock;
use proptest::prelude::*;
use std::{rc::Rc, sync::Arc};

/// Given a target address, we generate random calldata.
pub fn override_call_strat(
    fuzz_state: EvmFuzzState,
    contracts: FuzzRunIdentifiedContracts,
    target: Arc<RwLock<Address>>,
    calldata_fuzz_config: CalldataFuzzDictionary,
) -> SBoxedStrategy<(Address, Bytes)> {
    let contracts_ref = contracts.clone();
    proptest::prop_oneof![
        80 => proptest::strategy::LazyJust::new(move || *target.read()),
        20 => any::<prop::sample::Selector>()
            .prop_map(move |selector| *selector.select(contracts_ref.lock().keys())),
    ]
    .prop_flat_map(move |target_address| {
        let fuzz_state = fuzz_state.clone();
        let calldata_fuzz_config = calldata_fuzz_config.clone();
        let (_, abi, functions) = &contracts.lock()[&target_address];
        let func = select_random_function(abi, functions);
        func.prop_flat_map(move |func| {
            fuzz_contract_with_calldata(&fuzz_state, &calldata_fuzz_config, target_address, func)
        })
    })
    .sboxed()
}

/// Creates the invariant strategy.
///
/// Given the known and future contracts, it generates the next call by fuzzing the `caller`,
/// `calldata` and `target`. The generated data is evaluated lazily for every single call to fully
/// leverage the evolving fuzz dictionary.
///
/// The fuzzed parameters can be filtered through different methods implemented in the test
/// contract:
///
/// `targetContracts()`, `targetSenders()`, `excludeContracts()`, `targetSelectors()`
pub fn invariant_strat(
    fuzz_state: EvmFuzzState,
    senders: SenderFilters,
    contracts: FuzzRunIdentifiedContracts,
    dictionary_weight: u32,
    calldata_fuzz_config: CalldataFuzzDictionary,
) -> impl Strategy<Value = BasicTxDetails> {
    // We only want to seed the first value, since we want to generate the rest as we mutate the
    // state
    generate_call(fuzz_state, senders, contracts, dictionary_weight, calldata_fuzz_config)
}

/// Strategy to generate a transaction where the `sender`, `target` and `calldata` are all generated
/// through specific strategies.
fn generate_call(
    fuzz_state: EvmFuzzState,
    senders: SenderFilters,
    contracts: FuzzRunIdentifiedContracts,
    dictionary_weight: u32,
    calldata_fuzz_config: CalldataFuzzDictionary,
) -> BoxedStrategy<BasicTxDetails> {
    let senders = Rc::new(senders);
    any::<prop::sample::Selector>()
        .prop_flat_map(move |selector| {
            let (contract, func) = {
                let contracts = contracts.lock();
                let contracts =
                    contracts.iter().filter(|(_, (_, abi, _))| !abi.functions.is_empty());
                let (&contract, (_, abi, functions)) = selector.select(contracts);

                let func = select_random_function(abi, functions);
                (contract, func)
            };

            let senders = senders.clone();
            let fuzz_state = fuzz_state.clone();
            let calldata_fuzz_config = calldata_fuzz_config.clone();
            func.prop_flat_map(move |func| {
                let sender = select_random_sender(&fuzz_state, senders.clone(), dictionary_weight);
                let contract =
                    fuzz_contract_with_calldata(&fuzz_state, &calldata_fuzz_config, contract, func);
                (sender, contract)
            })
        })
        .boxed()
}

/// Strategy to select a sender address:
/// * If `senders` is empty, then it's either a random address (10%) or from the dictionary (90%).
/// * If `senders` is not empty, a random address is chosen from the list of senders.
fn select_random_sender(
    fuzz_state: &EvmFuzzState,
    senders: Rc<SenderFilters>,
    dictionary_weight: u32,
) -> BoxedStrategy<Address> {
    if !senders.targeted.is_empty() {
        any::<prop::sample::Selector>()
            .prop_map(move |selector| *selector.select(&senders.targeted))
            .boxed()
    } else {
        proptest::prop_oneof![
            100 - dictionary_weight => fuzz_param(&alloy_dyn_abi::DynSolType::Address, None)
                .prop_map(move |addr| addr.as_address().unwrap())
                .boxed(),
            dictionary_weight => fuzz_param_from_state(&alloy_dyn_abi::DynSolType::Address, fuzz_state)
                .prop_map(move |addr| addr.as_address().unwrap())
                .boxed(),
        ]
        // Too many exclusions can slow down testing.
        .prop_filter("excluded sender", move |addr| !senders.excluded.contains(addr))
        .boxed()
    }
}

/// Strategy to select a random mutable function from the abi.
///
/// If `targeted_functions` is not empty, select one from it. Otherwise, take any
/// of the available abi functions.
fn select_random_function(
    abi: &JsonAbi,
    targeted_functions: &[Function],
) -> BoxedStrategy<Function> {
    if !targeted_functions.is_empty() {
        let targeted_functions = targeted_functions.to_vec();
        let selector = any::<prop::sample::Selector>()
            .prop_map(move |selector| selector.select(&targeted_functions).clone());
        selector.boxed()
    } else {
        let possible_funcs: Vec<Function> = abi
            .functions()
            .filter(|&func| {
                !matches!(
                    func.state_mutability,
                    alloy_json_abi::StateMutability::Pure | alloy_json_abi::StateMutability::View
                )
            })
            .cloned()
            .collect();
        let total_random = any::<prop::sample::Selector>()
            .prop_map(move |selector| selector.select(&possible_funcs).clone());
        total_random.boxed()
    }
}

/// Given a function, it returns a proptest strategy which generates valid abi-encoded calldata
/// for that function's input types.
pub fn fuzz_contract_with_calldata(
    fuzz_state: &EvmFuzzState,
    calldata_fuzz_config: &CalldataFuzzDictionary,
    contract: Address,
    func: Function,
) -> impl Strategy<Value = (Address, Bytes)> {
    // We need to compose all the strategies generated for each parameter in all possible
    // combinations.
    // `prop_oneof!` / `TupleUnion` `Arc`s for cheap cloning.
    #[allow(clippy::arc_with_non_send_sync)]
    prop_oneof![
        60 => fuzz_calldata_with_config(func.clone(), Some(calldata_fuzz_config)),
        40 => fuzz_calldata_from_state(func, fuzz_state),
    ]
    .prop_map(move |calldata| {
        trace!(input=?calldata);
        (contract, calldata)
    })
}
