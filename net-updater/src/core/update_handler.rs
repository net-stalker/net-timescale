use std::error::Error;
use std::sync::Arc;

use sqlx::Postgres;
use sqlx::Pool;

use net_core_api::api::envelope::envelope::Envelope;

#[async_trait::async_trait]
pub trait UpdateHandler: Sync + Send {
    async fn update(
        &self,
        connection_pool: Arc<Pool<Postgres>>,
        enveloped_request: Envelope,
    ) -> Result<Envelope, Box<dyn Error + Send + Sync>>;
    
    fn get_updating_request_type(&self) -> &'static str;
}