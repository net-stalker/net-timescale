use std::{error::Error, sync::Arc};

use async_trait::async_trait;
use net_core_api::api::envelope::envelope::Envelope;
use sqlx::Postgres;
#[async_trait]
pub trait InsertHandler: core::fmt::Debug + Sync + Send {
    async fn insert(&self, transaction: &mut sqlx::Transaction<'_, Postgres>, data_to_insert: Envelope) -> Result<(), Box<dyn Error + Send + Sync>>;
    fn get_insertable_data_type() -> &'static str where Self: Sized;
    fn get_data_type(&self) -> &'static str;
}

pub trait InsertHandlerCtor: core::fmt::Debug + Sync + Send {
    fn call(&self) -> Arc<dyn InsertHandler>;
}

