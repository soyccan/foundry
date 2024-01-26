use crate::schema::etherscan_source_code;
use diesel::Insertable;
use serde_json::Value as JsonValue;

#[derive(Insertable, Default)]
#[diesel(table_name = etherscan_source_code)]
#[diesel(check_for_backend(Pg))]
pub(crate) struct SourceCodeDBRow {
    pub(crate) bytecode_hash: String,
    pub(crate) address: String,
    pub(crate) verified: bool,
    pub(crate) source_code_file: Option<String>,
    pub(crate) source_code_files: Option<JsonValue>,
    pub(crate) source_code_language: Option<String>,
    pub(crate) source_code_settings: Option<JsonValue>,
    pub(crate) abi: Option<JsonValue>,
    pub(crate) contract_name: Option<String>,
    pub(crate) compiler_version: Option<String>,
    pub(crate) optimization_used: Option<i32>,
    pub(crate) runs: Option<i32>,
    pub(crate) constructor_arguments: Option<Vec<u8>>,
    pub(crate) evm_version: Option<String>,
    pub(crate) library: Option<String>,
    pub(crate) license_type: Option<String>,
    pub(crate) proxy: Option<bool>,
    pub(crate) implementation: Option<String>,
    pub(crate) swarm_source: Option<String>,
}
