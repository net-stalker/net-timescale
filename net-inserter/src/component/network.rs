use std::error::Error;

use async_trait::async_trait;
use net_core_api::{api::envelope::envelope::Envelope, core::typed_api::Typed};
use net_inserter_api::api::pcap_file::InsertPcapFileDTO;
use sqlx::Postgres;
use crate::core::insert_handler::{InsertHandler, InsertHandlerCtor};

#[derive(Default, Debug)]
pub struct NetworkInserter {}

impl NetworkInserter {}

#[async_trait]
impl InsertHandler for NetworkInserter {
    async fn insert(&self, transaction: &mut sqlx::Transaction<'_, Postgres>, data_to_insert: Envelope) -> Result<(), Box<dyn Error + Send + Sync>> {
        log::debug!("TODO: implement insert for PcapFileInserter");
        Ok(())
    }

    fn get_insertable_data_type() -> &'static str {
        InsertPcapFileDTO::get_data_type()
    }

    fn get_data_type(&self) -> &'static str {
        Self::get_insertable_data_type()
    }
}

#[derive(Debug, Default)]
pub struct NetworkInserterCtor {}

impl InsertHandlerCtor for NetworkInserterCtor {
    fn call(&self) -> std::sync::Arc<dyn InsertHandler> {
        std::sync::Arc::new(NetworkInserter::default())
    }
}

