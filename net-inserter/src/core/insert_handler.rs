use async_trait::async_trait;
use net_core_api::api::envelope::envelope::Envelope;
use sqlx::Postgres;

use super::insert_error::InsertError;
#[async_trait]
pub trait InsertHandler: core::fmt::Debug + Sync + Send {
    async fn insert(&self, transaction: &mut sqlx::Transaction<'_, Postgres>, data_to_insert: Envelope) -> Result<Option<Envelope>, InsertError>;
    fn get_insertable_data_type(&self) -> &'static str;
}
