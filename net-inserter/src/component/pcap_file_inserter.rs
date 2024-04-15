use std::{error::Error, os::unix::fs::PermissionsExt};
use async_trait::async_trait;
use net_core_api::{api::envelope::envelope::Envelope, core::{decoder_api::Decoder, typed_api::Typed}};
use net_inserter_api::api::pcap_file::InsertPcapFileDTO;
use sqlx::Postgres;
use tokio::{fs::File, io::AsyncWriteExt};
use uuid::Uuid;
use crate::{core::{insert_error::InsertError, insert_handler::{InsertHandler, InsertHandlerCtor}}, utils::network_packet_inserter};

#[derive(Default, Debug)]
pub struct PcapFileInserter {
    output_directory: String,
}

impl PcapFileInserter {
    fn new(output_directory: &str) -> Self {
        Self { output_directory: output_directory.to_string() }
    }

    async fn save_pcap_file_to(&self, pcap_file_path: &str, pcap_data: &[u8]) -> Result<usize, Box<dyn Error + Send + Sync>> {
        let mut file = File::create(pcap_file_path).await?;

        // Set permissions to wr for owner and read for others
        let metadata = file.metadata().await?;
        let mut permissions = metadata.permissions();
        permissions.set_mode(0o600);

        Ok(file.write(pcap_data).await?)
    }
}

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
        let pcap_file_name = Uuid::now_v7().to_string();
        let pcap_file_path = format!("{}/{}", &self.output_directory, &pcap_file_name);
        
        let data_packet_save_result = self.save_pcap_file_to(&pcap_file_path, pcap_data.get_data()).await;

        if let Err(e) = data_packet_save_result {
            log::error!("Error: {e}");
            return Err(e);
        }

        let network_packet = match crate::utils::decoder::Decoder::decode(pcap_data.get_data()).await {
            Ok(data) => data,
            Err(err_desc) => return Err(Box::new(InsertError::DecodePcapFile(err_desc)))
        };
        let insert_result = network_packet_inserter::insert_network_packet_transaction(
            transaction,
            tenant_id,
            &pcap_file_path,
            &network_packet
        ).await; 
        match insert_result {
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
pub struct PcapFileInserterCtor {
    pcaps_output_directory: String, 
}

impl PcapFileInserterCtor {
    pub fn new(pcaps_output_directory: &str) -> Self {
        Self { pcaps_output_directory: pcaps_output_directory.to_string() }
    }
}

impl InsertHandlerCtor for PcapFileInserterCtor {
    fn call(&self) -> std::sync::Arc<dyn InsertHandler> {
        std::sync::Arc::new(PcapFileInserter::new(&self.pcaps_output_directory))
    }
}
