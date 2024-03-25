use std::sync::Arc;

use net_token_verifier::fusion_auth::jwt_token::Jwt;
use sqlx::Pool;
use sqlx::Postgres;

use net_core_api::api::envelope::envelope::Envelope;

#[async_trait::async_trait]
pub trait Requester: Sync + Send {
    async fn request(
        &self,
        connection_pool: Arc<Pool<Postgres>>,
        data: Envelope,
        jwt: Jwt,
    ) -> Result<Envelope, String>;
    
    fn get_requesting_type(&self) -> &'static str;
}