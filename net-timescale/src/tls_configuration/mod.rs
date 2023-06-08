use postgres::NoTls;
use postgres_native_tls::MakeTlsConnector;
use r2d2_postgres::PostgresConnectionManager;

// TODO: try to remove enum Pool
pub enum Pool {
    NoTlsPool(r2d2::Pool<PostgresConnectionManager<NoTls>>),
    TlsPool(r2d2::Pool<PostgresConnectionManager<MakeTlsConnector>>)
}

pub trait ConnectionFactory {
    // TODO: use associated type instead of Pool
    fn create_pool(&self) -> Pool;
}

pub mod tls_factory;
pub mod no_tls_factory;