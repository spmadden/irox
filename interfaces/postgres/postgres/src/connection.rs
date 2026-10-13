// SPDX-License-Identifier: MIT
// Copyright 2026 IROX Contributors
//

use crate::error::Error;
use native_tls::{Certificate, Identity, TlsConnector, TlsConnectorBuilder};
use postgres_native_tls::MakeTlsConnector;
pub use postgres_native_tls::TlsStream;
pub use tokio_postgres::config;
use tokio_postgres::Config;
pub use tokio_postgres::Connection;
pub use tokio_postgres::Socket;

pub const ENV_TRY_CERTS: &str = "IROX_POSTGRES_TRY_CERTS";

pub struct DbConnBuilder {
    connector: TlsConnectorBuilder,
}
impl Default for DbConnBuilder {
    fn default() -> Self {
        Self {
            connector: TlsConnector::builder(),
        }
    }
}
impl DbConnBuilder {
    pub fn try_load_environments(self) -> Result<Self, Error> {
        if let Ok(try_certs) = std::env::var(ENV_TRY_CERTS) {
            if try_certs == "true" {
                return self.with_identity_cert_home();
            }
        }
        Ok(self)
    }

    #[cfg(target_os = "windows")]
    pub fn with_identity_cert_home(self) -> Result<Self, Error> {
        let ad = std::env::var("APPDATA")?;
        let mycert = std::path::PathBuf::new()
            .join(&ad)
            .join("postgresql")
            .join("postgresql.crt");
        let mykey = std::path::PathBuf::new()
            .join(&ad)
            .join("postgresql")
            .join("postgresql.key");
        let root = std::path::PathBuf::new()
            .join(ad)
            .join("postgresql")
            .join("root.crt");
        let mycert = std::fs::read(&mycert)?;
        let mykey = std::fs::read(&mykey)?;
        let root = std::fs::read(&root)?;
        Ok(self
            .with_root_cert(&root)?
            .with_identity_cert(Identity::from_pkcs8(&mycert, &mykey)?))
    }
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub fn with_identity_cert_home(mut self) -> Result<Self, Error> {
        let ad = std::env::var("HOME")?;
        let mycert = std::path::PathBuf::new()
            .join(&ad)
            .join("postgresql")
            .join("postgresql.crt");
        let mykey = std::path::PathBuf::new()
            .join(&ad)
            .join("postgresql")
            .join("postgresql.key");
        let root = std::path::PathBuf::new()
            .join(ad)
            .join("postgresql")
            .join("root.crt");
        let mycert = std::fs::read(&mycert)?;
        let mykey = std::fs::read(&mykey)?;
        let root = std::fs::read(&root)?;
        self.with_root_cert(&root)?
            .with_identity_cert(Identity::from_pkcs8(&mycert, &mykey)?);
        Ok(self)
    }
    ///
    /// Use postgres standard `PGSSLCERT` and `PGSSLKEY` environment variables to
    /// populate an identity certificate.  Must be in PEM format.
    pub fn with_identity_cert_env(self) -> Result<Self, Error> {
        let mycert = std::env::var("PGSSLCERT")?;
        let mykey = std::env::var("PGSSLKEY")?;
        let root = std::env::var("PGSSLROOTCERT")?;
        let mycert = std::fs::read(&mycert)?;
        let mykey = std::fs::read(&mykey)?;
        let root = std::fs::read(&root)?;

        Ok(self
            .with_root_cert(&root)?
            .with_identity_cert(Identity::from_pkcs8(&mycert, &mykey)?))
    }
    pub fn with_identity_cert(mut self, identity: Identity) -> Self {
        self.connector.identity(identity);
        self
    }
    ///
    /// Adds a root certificate to the trusted list
    pub fn with_root_cert(mut self, cert: &[u8]) -> Result<Self, Error> {
        self.connector
            .add_root_certificate(Certificate::from_pem(cert)?);
        Ok(self)
    }
    pub async fn build_and_connect(
        self,
        config: config::Config,
    ) -> Result<(DbConn, Connection<Socket, TlsStream<Socket>>), Error> {
        let connector = self.connector.build()?;
        let connector = MakeTlsConnector::new(connector);
        let (client, connection) = config.connect(connector).await?;

        Ok((DbConn { client }, connection))
    }
}
///
/// Basic database connection wrapper, most functions will utilize the internal client directly.
pub struct DbConn {
    pub client: tokio_postgres::Client,
}
impl DbConn {
    ///
    /// Quick connection test to ensure the connection is still alive.
    pub async fn ping(&mut self) -> Result<(), Error> {
        Ok(self.client.check_connection().await?)
    }
}
macro_rules! config_item {
    ($config:ident,$setter:ident,$env:literal) => {
        if let Ok(var) = std::env::var($env) {
            $config.$setter(var);
        }
    };
    ($config:ident,$setter:ident,$env:literal,$parsed:ty) => {
        if let Ok(var) = std::env::var($env) {
            if let Ok(parsed) = var.parse::<$parsed>() {
                $config.$setter(parsed.into());
            }
        }
    };
}
#[allow(clippy::upper_case_acronyms)]
struct SSLMODE(config::SslMode);
impl From<SSLMODE> for config::SslMode {
    fn from(value: SSLMODE) -> Self {
        value.0
    }
}
impl core::str::FromStr for SSLMODE {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "disable" => Ok(SSLMODE(config::SslMode::Disable)),
            "allow" | "prefer" => Ok(SSLMODE(config::SslMode::Prefer)),
            "require" | "verify-ca" | "verify-full" => Ok(SSLMODE(config::SslMode::Require)),
            _ => Err(()),
        }
    }
}
///
/// Attempts to read configuration from environment variables.  If no environment variables are
/// found, returns a default configuration.
pub fn config_from_env() -> Config {
    let mut config = Config::new();
    config_item!(config, user, "PGUSER");
    config_item!(config, password, "PGPASSWORD");
    config_item!(config, dbname, "PGDATABASE");
    config_item!(config, options, "PGOPTIONS");
    config_item!(config, application_name, "PGAPPNAME");
    config_item!(config, ssl_mode, "PGSSLMODE", SSLMODE);
    config_item!(config, host, "PGHOST");
    config_item!(config, port, "PGPORT", u16);
    config
}
