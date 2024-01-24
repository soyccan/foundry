use crate::schema::{etherscan_source_code, no_source_code};
use diesel::Insertable;
use serde_json::Value as JsonValue;

#[derive(Insertable)]
#[diesel(table_name = etherscan_source_code)]
#[diesel(check_for_backend(Pg))]
pub(crate) struct SourceCodeDBRow {
    pub(crate) bytecode_hash: String,
    pub(crate) address: String,
    pub(crate) source_code_file: Option<String>,
    pub(crate) source_code_files: Option<JsonValue>,
    pub(crate) source_code_language: Option<String>,
    pub(crate) source_code_settings: Option<JsonValue>,
    pub(crate) abi: JsonValue,
    pub(crate) contract_name: String,
    pub(crate) compiler_version: String,
    pub(crate) optimization_used: i32,
    pub(crate) runs: i32,
    pub(crate) constructor_arguments: Vec<u8>,
    pub(crate) evm_version: String,
    pub(crate) library: String,
    pub(crate) license_type: String,
    pub(crate) proxy: bool,
    pub(crate) implementation: Option<String>,
    pub(crate) swarm_source: String,
}

#[derive(Insertable)]
#[diesel(table_name = no_source_code)]
#[diesel(check_for_backend(Pg))]
pub(crate) struct NoSourceCodeDBRow {
    pub(crate) bytecode_hash: String,
}
