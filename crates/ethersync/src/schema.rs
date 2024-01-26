// @generated automatically by Diesel CLI.

diesel::table! {
    etherscan_source_code (id) {
        id -> Int4,
        date_fetched -> Date,
        bytecode_hash -> Text,
        address -> Text,
        source_code_file -> Nullable<Text>,
        source_code_files -> Nullable<Jsonb>,
        source_code_language -> Nullable<Text>,
        source_code_settings -> Nullable<Jsonb>,
        abi -> Nullable<Jsonb>,
        contract_name -> Nullable<Text>,
        compiler_version -> Nullable<Text>,
        optimization_used -> Nullable<Int4>,
        runs -> Nullable<Int4>,
        constructor_arguments -> Nullable<Bytea>,
        evm_version -> Nullable<Text>,
        library -> Nullable<Text>,
        license_type -> Nullable<Text>,
        proxy -> Nullable<Bool>,
        implementation -> Nullable<Text>,
        swarm_source -> Nullable<Text>,
        verified -> Bool,
    }
}
