use crate::ProxyInfo;
use diesel::{
    connection::DefaultLoadingMode, table, Connection as dConnection, ExpressionMethods, QueryDsl,
    QueryResult, RunQueryDsl,
};
use eyre::{Result, WrapErr};
use futures::stream::Stream;
use sqlx::Connection as sConnection;

pub struct ProxionDatabase {
    connection: diesel::PgConnection,
}

pub struct ContractsDatabase {
    connection: sqlx::PgConnection,
}

table! {
    contracts (id) {
        id -> Int4,
        address -> Text,
        block -> Int4,
        year -> Int2,
        bytecode_hash -> Nullable<Text>,
        self_destructed -> Bool,
    }
}

impl ProxionDatabase {
    pub fn connect(database_url: &str) -> Result<Self> {
        Ok(Self {
            connection: diesel::PgConnection::establish(database_url)
                .wrap_err_with(|| "Failed to connect to the database")?,
        })
    }

    pub fn get_alive_contracts(
        &mut self,
    ) -> Result<impl Iterator<Item = QueryResult<(String, Option<String>)>>> {
        // TODO: diesel does not support server-side cursor and row streaming
        contracts::table
            .select((contracts::address, contracts::bytecode_hash))
            .filter(contracts::self_destructed.eq(false))
            .load_iter::<_, DefaultLoadingMode>(&mut self.connection)
            .wrap_err_with(|| "Failed to get contracts")
    }
}

table! {
    proxy_info (address, block) {
        address -> Text,
        block -> Integer,
        erc1167_minimal -> Nullable<Bool>,
    }
}

impl ProxyInfo for ProxionDatabase {
    fn is_minimal_proxy(&mut self, address: &str) -> Result<bool> {
        proxy_info::table
            .select(proxy_info::erc1167_minimal)
            .filter(proxy_info::address.eq(address))
            .order(proxy_info::block.desc())
            .first::<Option<bool>>(&mut self.connection)
            .map(|x| x.unwrap_or(false))
            .wrap_err_with(|| format!("Failed to query minimal proxy: {}", address))
    }
}

impl ContractsDatabase {
    pub async fn connect(database_url: &str) -> Result<Self> {
        Ok(Self {
            connection: sqlx::PgConnection::connect(database_url)
                .await
                .wrap_err_with(|| "Failed to connect to the database")?,
        })
    }

    pub fn get_alive_contracts(
        &mut self,
    ) -> impl Stream<Item = sqlx::Result<sqlx::postgres::PgRow>> + '_ {
        sqlx::query("SELECT address, bytecode_hash FROM contracts WHERE NOT self_destructed")
            .fetch(&mut self.connection)
    }
}
