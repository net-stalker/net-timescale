use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

use sqlx::Pool;
use sqlx::Postgres;

use net_core_api::api::envelope::envelope::Envelope;

use crate::core::update_handler::UpdateHandler;

use super::builder::UpdateManagerBuilder;


pub struct UpdateManager {
    update_handlers: HashMap<&'static str, Box<dyn UpdateHandler>>
}

impl UpdateManager {
    pub fn new(
        updaters: HashMap<&'static str, Box<dyn UpdateHandler>>
    ) -> Self {
        Self {
            update_handlers: updaters
        }
    }

    pub fn builder() -> UpdateManagerBuilder {
        UpdateManagerBuilder::default()
    }

    pub async fn handle_update(
        &self,
        enveloped_request: Envelope,
        connection_pool: Arc<Pool<Postgres>>
    ) -> Result<Envelope, Box<dyn Error + Send + Sync>> {
        let updater = self.update_handlers.get(enveloped_request.get_envelope_type());
        if updater.is_none() {
            return Err("Error: Tere is no such request available".into());
        }
        let updater = updater.unwrap().as_ref();
        updater.update(connection_pool, enveloped_request).await
    }
}