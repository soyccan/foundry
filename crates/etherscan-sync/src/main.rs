use dotenvy::dotenv;
use etherscan_sync::{
    proxion::{ContractsDatabase, ProxionDatabase},
    Etherscan, EtherscanSync, SourceCodeDatabase,
};
use eyre::{eyre, Result};
use futures::TryStreamExt;
use log::{error, info};
use sqlx::Row;
use std::{env, time::Instant};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    dotenv().ok();

    let etherscan = Etherscan::new(get_env_var("ETHERSCAN_API_KEY")?.as_str())?;
    let mut source_code_database =
        SourceCodeDatabase::connect(get_env_var("DATABASE_URL")?.as_str())?;
    let mut proxion_database =
        ProxionDatabase::connect(get_env_var("DATABASE_URL_PROXION")?.as_str())?;
    let mut etherscan_sync =
        EtherscanSync::new(&etherscan, &mut source_code_database, &mut proxion_database);

    let mut contracts_database =
        ContractsDatabase::connect(get_env_var("DATABASE_URL_PROXION")?.as_str()).await?;

    info!("Getting alive contracts");
    let start_time = Instant::now();
    let mut alive_contracts = contracts_database.get_alive_contracts();
    info!("Got ? alive contracts in {}s", start_time.elapsed().as_secs());

    while let Some(row) = alive_contracts.try_next().await? {
        if let (Ok(address), Ok(bytecode_hash)) =
            (row.try_get::<String, _>(0), row.try_get::<Option<String>, _>(1))
        {
            let bytecode_hash = match bytecode_hash {
                Some(bytecode_hash) => bytecode_hash,
                None => {
                    error!("Null bytecode hash: address={}", address);
                    continue;
                }
            };
            if let Err(e) = etherscan_sync
                .sync_source_code_to_database(address.as_str(), bytecode_hash.as_str())
                .await
            {
                error!(
                    "Failed to sync source code: address={} bytecode_hash={}: {:#?}",
                    address, bytecode_hash, e
                );
                continue;
            }
        }
    }
    Ok(())
}

fn get_env_var(key: &str) -> Result<String> {
    env::var(key).map_err(|e| eyre!(format!("failed to get environment variable `{}`: {}", key, e)))
}

fn _get_address_list_for_test() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "0xd4eb55b3a6dfd86df71a974044109490dd4d1480",
            "0xb7d43352b73a736432efad8ec584a23ae142522a0bbf506332c4f1ee52b76cc7",
        ), // not verified
        (
            "0xa0d37754fa818dd78fc17d8923ab82329d61cfb8",
            "0x1b460c826a854d61dca82f718e088b8b4c4082ffeb93752d7691bc62c51dc028",
        ), // minimal proxy
        (
            "0xf307c0907f1c356f36a35ba6436c678267b251ef",
            "0x59dbd1d383660656b50767856918a28c79a4f25e4857c5b9ffcbb67997014f58",
        ), // ERC-1967 proxy
        (
            "0x5cbc0491e2f58409b44404a3ac50d335cb318015",
            "0x0001670fb2d97841dfcdc7a208d7b377d40e7a56619292feea47b253c8b9e4ae",
        ), // single source file
        (
            "0xca878cf4a27690637c07b39ae06d26f7679be4fc",
            "0x000dcda24880c9156ef630ec0dd719e43c0f9664900e70cd79c703944487770d",
        ), // multiple source files
        (
            "0x42bb6d1bb09959a61cc1d1d98ccc7902dfde3e92",
            "0x677245fd7a59fd65a4238648b497775b88a2fa4773a2848131b969bd3c26e75c",
        ), // multiple source files with metadata
    ]
}
