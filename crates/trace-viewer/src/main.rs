use alloy_primitives::{Address, Bytes, U256};
use clap::Parser;
use eyre::Result;
use foundry_debugger::DebuggerArgs;
use foundry_evm::inspectors::Debugger;
use foundry_evm_core::debug::{DebugStep, Instruction};
use hex::FromHex;
use revm::interpreter::instructions::opcode::{self, OPCODE_JUMPMAP};
use serde::{Deserialize, Deserializer};

#[derive(Debug, Parser)]
struct Args {
    trace_file: String,
}

#[derive(Debug, Deserialize, Clone)]
struct GethStructLog {
    pc: u64,
    op: String,
    gas: u64,

    // #[serde(rename = "gasCost")]
    // gas_cost: u64,
    ///
    #[serde(deserialize_with = "deserialize_hex")]
    memory: Vec<u8>,

    // #[serde(rename = "memSize")]
    // mem_size: i32,
    ///
    stack: Vec<U256>,

    // #[serde(rename = "returnData", default)]
    // return_data: Vec<u8>,

    // #[serde(default)]
    // storage: Vec<(U256, U256)>,
    ///
    depth: i32,

    // #[serde(default)]
    // refund: u64,

    // #[serde(default)]
    // error: Option<String>,
    ///

    ///
    /// address and code_address are extensions to the Geth trace format
    // address: Address,

    #[serde(rename = "codeAddr")]
    code_address: Address,
}

impl Into<DebugStep> for GethStructLog {
    fn into(self) -> DebugStep {
        DebugStep {
            stack: self.stack,
            memory: self.memory,
            instruction: Instruction::OpCode(
                OPCODE_JUMPMAP
                    .iter()
                    .position(|&x| x == Some(&self.op))
                    .unwrap_or(opcode::INVALID as usize) as u8,
            ),
            push_bytes: None,
            pc: self.pc as usize,
            total_gas_used: self.gas,
        }
    }
}

fn deserialize_hex<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Bytes::from_hex(s).map_err(serde::de::Error::custom).map(Into::into)
}

fn main() -> Result<()> {
    let args = Args::parse();
    let content = std::fs::read_to_string(args.trace_file)?;
    let mut trace: Vec<Option<GethStructLog>> = serde_json::from_str(&content)?;

    let mut prev_step: Option<GethStructLog> = None;
    let mut debugger = Debugger::default();
    trace.iter_mut().for_each(|step| {
        let step = match step.take() {
            Some(step) => step,
            None => return,
        };
        if let Some(prev_step) = prev_step.take() {
            if step.depth > prev_step.depth {
                debugger.enter(
                    prev_step.depth as usize,
                    prev_step.code_address,
                    Default::default(),
                );
            }
            if step.depth < prev_step.depth {
                debugger.exit();
            }
        } else {
            debugger.enter(0, step.code_address, Default::default());
        }
        prev_step = Some(step.clone());
        debugger.arena.arena[debugger.head].steps.push(step.into());
    });
    debugger.exit();

    let debugger = DebuggerArgs {
        debug: vec![debugger.arena],
        decoder: &Default::default(),
        sources: Default::default(),
        breakpoints: Default::default(),
    };
    debugger.run()?;

    Ok(())
}
