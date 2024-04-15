use std::error::Error;
use async_trait::async_trait;
use net_core_api::{api::envelope::envelope::Envelope, core::{decoder_api::Decoder, typed_api::Typed}};
use net_inserter_api::api::pcap_file::InsertPcapFileDTO;
use sqlx::Postgres;
use crate::{core::{insert_error::InsertError, insert_handler::{InsertHandler, InsertHandlerCtor}}, utils::network_packet_inserter};

#[derive(Default, Debug)]
pub struct PcapFileInserter {}

impl PcapFileInserter {}

#[async_trait]
impl InsertHandler for PcapFileInserter {
    async fn insert(&self, transaction: &mut sqlx::Transaction<'_, Postgres>, data_to_insert: Envelope) -> Result<(), Box<dyn Error + Send + Sync>> {
        if data_to_insert.get_envelope_type() != Self::get_insertable_data_type() {
            return Err(Box::new(InsertError::WrongInsertableData(
                Self::get_insertable_data_type()
                .split('_')
                .collect::<Vec<_>>()
                .join(" ")
            )))
        }
        let tenant_id = data_to_insert.get_tenant_id();
        let pcap_data = InsertPcapFileDTO::decode(data_to_insert.get_data());
        let network_packet = match crate::utils::decoder::Decoder::decode(pcap_data.get_data()).await {
            Ok(data) => data,
            Err(err_desc) => return Err(Box::new(InsertError::DecodePcapFile(err_desc)))
        };
        match network_packet_inserter::insert_network_packet_transaction(
            transaction, 
            tenant_id, 
            "MOCK_AGENT_ID", 
            &network_packet
        ).await {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(InsertError::DbError(Self::get_insertable_data_type().to_string(), e))),
        }
    }

    fn get_insertable_data_type() -> &'static str {
        InsertPcapFileDTO::get_data_type()
    }

    fn get_data_type(&self) -> &'static str {
        Self::get_insertable_data_type()
    }
}

#[derive(Debug, Default)]
pub struct PcapFileInserterCtor {}

impl InsertHandlerCtor for PcapFileInserterCtor {
    fn call(&self) -> std::sync::Arc<dyn InsertHandler> {
        std::sync::Arc::new(PcapFileInserter::default())
    }
}
