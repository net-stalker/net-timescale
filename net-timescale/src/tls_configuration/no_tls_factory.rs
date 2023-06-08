use std::sync::Arc;
use postgres::NoTls;
use r2d2_postgres::PostgresConnectionManager;
use crate::tls_configuration::{ConnectionFactory, Pool};

pub struct NoTlsConnectionFactory {
    connection_string: String,
    max_connection_size: u32,
}

impl NoTlsConnectionFactory {
    pub fn builder() -> NoTlsConnectionFactoryBuilder {
        NoTlsConnectionFactoryBuilder::new()
    }
    pub fn into_inner(self) -> Arc<Self> {
        Arc::new(self)
    }
}

pub struct NoTlsConnectionFactoryBuilder {
    connection_string: String,
    max_connection_size: u32,
}

impl NoTlsConnectionFactoryBuilder {
    pub fn new() -> Self {
        NoTlsConnectionFactoryBuilder {
            connection_string: "".to_string(),
            max_connection_size: 0,
        }
    }
    pub fn with_max_connection_size(mut self, connection_size: u32) -> Self {
        self.max_connection_size = connection_size;
        self
    }
    pub fn with_connection_string(mut self, connection_string: String) -> Self {
        self.connection_string = connection_string;
        self
    }
    pub fn build(self) -> NoTlsConnectionFactory {
        NoTlsConnectionFactory {
            connection_string: self.connection_string,
            max_connection_size: self.max_connection_size,
        }
    }
}

impl ConnectionFactory for NoTlsConnectionFactory {
    fn create_pool(&self) -> Pool {
        let connection_manager = PostgresConnectionManager::new(
            self.connection_string.parse().unwrap(),
            NoTls,
        );
        Pool::NoTlsPool(r2d2::Pool::builder().max_size(self.max_connection_size).build(connection_manager).unwrap())
    }
}

// TODO: add tests (need to investigate how to run timescaleDB in CI)