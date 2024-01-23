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
        abi -> Jsonb,
        contract_name -> Text,
        compiler_version -> Text,
        optimization_used -> Int4,
        runs -> Int4,
        constructor_arguments -> Bytea,
        evm_version -> Text,
        library -> Text,
        license_type -> Text,
        proxy -> Bool,
        implementation -> Nullable<Text>,
        swarm_source -> Text,
    }
}

diesel::table! {
    no_source_code (bytecode_hash) {
        bytecode_hash -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    etherscan_source_code,
    no_source_code,
);
