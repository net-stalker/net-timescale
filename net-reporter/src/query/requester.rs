use std::sync::Arc;

use sqlx::Pool;
use sqlx::Postgres;

use net_core_api::envelope::envelope::Envelope;

#[async_trait::async_trait]
pub trait Requester: Sync + Send {
    async fn request(
        &self,
        connection_pool: Arc<Pool<Postgres>>,
        data: Envelope
    ) -> Result<Envelope, String>;
    
    fn get_requesting_type(&self) -> &'static str;
}