use crate::ProxyInfo;
use diesel::{table, Connection, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use eyre::{Result, WrapErr};

pub struct ProxionDatabase {
    connection: PgConnection,
}

impl ProxionDatabase {
    pub fn connect(database_url: &str) -> Result<Self> {
        Ok(Self {
            connection: PgConnection::establish(database_url)
                .wrap_err_with(|| "Failed to connect to the database")?,
        })
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
