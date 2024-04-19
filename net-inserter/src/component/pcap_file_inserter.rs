use std::error::Error;
use std::os::unix::fs::PermissionsExt;
use async_trait::async_trait;
use net_core_api::api::envelope::envelope::Envelope;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::typed_api::Typed;
use net_inserter_api::api::pcap_file::InsertPcapFileDTO;
use sqlx::Postgres;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;
use crate::core::insert_error::InsertError;
use crate::core::insert_handler::InsertHandler;
use crate::utils::network_packet_inserter;

#[derive(Default, Debug)]
pub struct InsertPcapFileHandler {
    output_directory: String,
}

impl InsertPcapFileHandler {
    pub fn new(output_directory: &str) -> Self {
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
impl InsertHandler for InsertPcapFileHandler {
    async fn insert(&self, transaction: &mut sqlx::Transaction<'_, Postgres>, data_to_insert: Envelope) -> Result<Option<Envelope>, Box<dyn Error + Send + Sync>> {
        if data_to_insert.get_envelope_type() != self.get_insertable_data_type() {
            return Err(Box::new(InsertError::WrongInsertableData(
                self.get_insertable_data_type()
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

        let network_packet_data = match crate::utils::decoder::Decoder::get_network_packet_data(pcap_data.get_data()).await {
            Ok(data) => data,
            Err(err_desc) => return Err(Box::new(InsertError::DecodePcapFile(err_desc)))
        };
        let insert_result = network_packet_inserter::insert_network_packet_transaction(
            transaction,
            tenant_id,
            &pcap_file_path,
            &network_packet_data
        ).await; 
        match insert_result {
            Ok(res) => Ok(Some(Envelope::new(
                tenant_id,
                res.get_type(),
                &res.encode(),
            ))),
            Err(e) => Err(Box::new(InsertError::DbError(self.get_insertable_data_type().to_string(), e))),
        }
    }

    fn get_insertable_data_type(&self) -> &'static str {
        InsertPcapFileDTO::get_data_type()
    }
}
