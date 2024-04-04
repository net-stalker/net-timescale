use std::error::Error;
use std::sync::Arc;

use sqlx::Pool;
use sqlx::Postgres;

use net_core_api::api::envelope::envelope::Envelope;

#[async_trait::async_trait]
pub trait Requester: Sync + Send {
    async fn request_enveloped_chart(
        &self,
        connection_pool: Arc<Pool<Postgres>>,
        data: Envelope,
    ) -> Result<Envelope, Box<dyn Error + Send + Sync>>;
    
    fn get_requesting_type(&self) -> &'static str;
}