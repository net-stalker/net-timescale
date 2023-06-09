use std::sync::Arc;
use native_tls::{Identity, TlsConnector};
use postgres_native_tls::MakeTlsConnector;
use r2d2_postgres::PostgresConnectionManager;
use crate::tls_configuration::{ConnectionFactory, Pool};

pub struct TlsConnectionFactory {
    client_key: Vec<u8>,
    client_crt: Vec<u8>,
    connection_string: String,
    invalid_certs: bool,
    max_connection_size: u32
}

impl TlsConnectionFactory {
    pub fn builder() -> TlsConnectionFactoryBuilder {
        TlsConnectionFactoryBuilder::new()
    }
    pub fn into_inner(self) -> Arc<Self> {
        Arc::new(self)
    }
}

pub struct TlsConnectionFactoryBuilder {
    path_to_client_key: String,
    path_to_client_crt: String,
    connection_string: String,
    invalid_certs: bool,
    max_connection_size: u32,
}

impl TlsConnectionFactoryBuilder {
    pub fn new() -> Self {
        TlsConnectionFactoryBuilder {
            path_to_client_key: "".to_string(),
            path_to_client_crt: "".to_string(),
            connection_string: "".to_string(),
            invalid_certs: false,
            max_connection_size: 0,
        }
    }
    pub fn with_crt(mut self, path_to_crt: String) -> Self {
        self.path_to_client_crt = path_to_crt;
        self
    }
    pub fn with_key(mut self, path_to_key: String) -> Self {
        self.path_to_client_key = path_to_key;
        self
    }
    pub fn with_max_connection_size(mut self, connection_size: u32) -> Self {
        self.max_connection_size = connection_size;
        self
    }
    pub fn with_connection_string(mut self, connection_string: String) -> Self {
        self.connection_string = connection_string;
        self
    }
    pub fn accept_invalid_certs(mut self, accept_invalid_certs: bool) -> Self {
        self.invalid_certs = accept_invalid_certs;
        self
    }
    pub fn build(self) -> TlsConnectionFactory {
        let crt = std::fs::read(self.path_to_client_crt).unwrap();
        let key = std::fs::read(self.path_to_client_key).unwrap();

        TlsConnectionFactory {
            client_key: key,
            client_crt: crt,
            connection_string: self.connection_string,
            invalid_certs: self.invalid_certs,
            max_connection_size: self.max_connection_size,
        }
    }
}

impl ConnectionFactory for TlsConnectionFactory {
    fn create_pool(&self) -> Pool {
        let client = Identity::from_pkcs8(&self.client_crt, &self.client_key).unwrap();
        let connector = TlsConnector::builder()
            .danger_accept_invalid_certs(self.invalid_certs)
            .identity(client)
            .build()
            .unwrap();
        let tls_connector = MakeTlsConnector::new(connector);
        let connection_manager = PostgresConnectionManager::new(
            self.connection_string.parse().unwrap(),
            tls_connector,
        );
        Pool::TlsPool(r2d2::Pool::builder().max_size(self.max_connection_size).build(connection_manager).unwrap())
    }
}

// TODO: add tests
