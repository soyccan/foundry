mod models;
pub mod proxion;
mod schema;

use crate::{
    models::{NoSourceCodeDBRow, SourceCodeDBRow},
    schema::{etherscan_source_code, no_source_code},
};
use diesel::{Connection, PgConnection, RunQueryDsl};
use eyre::{Result, WrapErr};
use foundry_block_explorers::{
    contract::{ContractMetadata, Metadata, SourceCodeLanguage, SourceCodeMetadata},
    errors::EtherscanError,
    Client,
};
use log::{debug, info};
use serde_json::json;

pub struct EtherSync<'a, PI: ProxyInfo> {
    etherscan: &'a Etherscan,
    source_code_database: &'a mut SourceCodeDatabase,
    proxy_info: &'a mut PI,
}

pub struct Etherscan {
    client: Client,
}

pub struct SourceCodeDatabase {
    connection: PgConnection,
}

pub trait ProxyInfo: Sized {
    fn is_minimal_proxy(&mut self, address: &str) -> Result<bool>;
}

impl<'a, PI: ProxyInfo> EtherSync<'a, PI> {
    pub fn new(
        etherscan: &'a Etherscan,
        source_code_database: &'a mut SourceCodeDatabase,
        proxy_info: &'a mut PI,
    ) -> Self {
        Self { etherscan, source_code_database, proxy_info }
    }

    pub async fn sync_source_code_to_database(
        &mut self,
        address: &str,
        bytecode_hash: &str,
    ) -> Result<()> {
        debug!("Syncing source code: address={} bytecode_hash={}", address, bytecode_hash);

        if self.proxy_info.is_minimal_proxy(address)? {
            debug!("Skipping minimal proxy: address={} bytecode_hash={}", address, bytecode_hash);
            self.source_code_database.set_no_source_code(bytecode_hash)?;
            return Ok(());
        }

        info!("Fetching source code: address={} bytecode_hash={}", address, bytecode_hash);
        match self.etherscan.get_source_code(address).await {
            Ok(source_code) => {
                debug!("Saving source code: address={} bytecode_hash={}", address, bytecode_hash);
                self.source_code_database.save_source_code(address, bytecode_hash, source_code)?;
            }
            Err(e) => {
                if let Some(EtherscanError::ContractCodeNotVerified(_)) = e.downcast_ref() {
                    debug!(
                        "Source code not verified: address={} bytecode_hash={}",
                        address, bytecode_hash
                    );
                    self.source_code_database.set_no_source_code(bytecode_hash)?;
                } else {
                    return Err(e);
                }
            }
        }
        Ok(())
    }
}

impl Etherscan {
    pub fn new(api_key: &str) -> Result<Self> {
        // // load the Foundry config file at ~/.foundry/foundry.toml
        // let config = Config::load();
        // let chain = config.chain.unwrap_or_default();
        // let api_key =
        //     config.get_etherscan_api_key(Some(chain)).ok_or_eyre("No Etherscan API key found")?;
        Ok(Self {
            client: Client::new(Default::default(), api_key)
                .wrap_err_with(|| "Failed to create Etherscan client")?,
        })
    }

    pub async fn get_source_code(&self, address: &str) -> Result<ContractMetadata> {
        self.client
            .contract_source_code(address.parse()?)
            .await
            .wrap_err_with(|| "Failed to get source code")
    }
}

impl SourceCodeDatabase {
    pub fn connect(database_url: &str) -> Result<Self> {
        Ok(Self {
            connection: PgConnection::establish(database_url)
                .wrap_err_with(|| "Failed to connect to the database")?,
        })
    }

    pub fn save_source_code(
        &mut self,
        address: &str,
        bytecode_hash: &str,
        source_code: ContractMetadata,
    ) -> Result<()> {
        for item in source_code.items {
            self.save_source_code_one(address, bytecode_hash, item)?;
        }
        Ok(())
    }

    fn save_source_code_one(
        &mut self,
        address: &str,
        bytecode_hash: &str,
        source_code: Metadata,
    ) -> Result<()> {
        let (source_code_file, source_code_files, source_code_language, source_code_settings) =
            match source_code.source_code {
                SourceCodeMetadata::SourceCode(source_file) => {
                    (Some(source_file), None, None, None)
                }
                SourceCodeMetadata::Sources(source_files) => {
                    (None, Some(json!(source_files)), None, None)
                }
                SourceCodeMetadata::Metadata { language, sources, settings } => (
                    None,
                    Some(json!(sources)),
                    language.map(|language| match language {
                        SourceCodeLanguage::Solidity => "Solidity".to_owned(),
                        SourceCodeLanguage::Vyper => "Vyper".to_owned(),
                    }),
                    settings.map(|settings| json!(settings)),
                ),
            };
        diesel::insert_into(etherscan_source_code::table)
            .values(SourceCodeDBRow {
                bytecode_hash: bytecode_hash.to_owned(),
                address: address.to_owned(),
                source_code_file,
                source_code_files,
                source_code_language,
                source_code_settings,
                abi: serde_json::from_str(source_code.abi.as_str())?,
                contract_name: source_code.contract_name,
                compiler_version: source_code.compiler_version,
                optimization_used: source_code.optimization_used as i32,
                runs: source_code.runs as i32,
                constructor_arguments: source_code.constructor_arguments.into(),
                evm_version: source_code.evm_version,
                library: source_code.library,
                license_type: source_code.license_type,
                proxy: source_code.proxy > 0,
                // the debug form of alloy_primitives::Address is lowercase hex without checksum
                implementation: source_code.implementation.map(|x| format!("{:?}", x)),
                swarm_source: source_code.swarm_source,
            })
            .on_conflict_do_nothing()
            .execute(&mut self.connection)
            .map(|_nrows| ())
            .wrap_err_with(|| "Failed to save source code into database")
    }

    pub fn set_no_source_code(&mut self, bytecode_hash: &str) -> Result<()> {
        diesel::insert_into(no_source_code::table)
            .values(NoSourceCodeDBRow { bytecode_hash: bytecode_hash.to_owned() })
            .on_conflict_do_nothing()
            .execute(&mut self.connection)
            .map(|_nrows| ())
            .wrap_err_with(|| {
                format!("Failed to write to no_source table: (bytecode_hash={},)", bytecode_hash)
            })
    }
}
