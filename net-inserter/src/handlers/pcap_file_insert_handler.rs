use std::error::Error;
use std::os::unix::fs::PermissionsExt;
use std::sync::Arc;
use async_trait::async_trait;
use net_component::handler::network_service_handler::NetworkServiceHandler;
use net_core_api::api::envelope::envelope::Envelope;
use net_core_api::api::result::result::ResultDTO;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::typed_api::Typed;
use net_inserter_api::api::pcap_file::InsertPcapFileDTO;
use sqlx::{Pool, Postgres};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;
use crate::core::insert_error::InsertError;
use crate::utils::network_packet_inserter;

#[derive(Default, Debug)]
pub struct InsertPcapFileHandler {
    output_directory: String,
}

impl InsertPcapFileHandler {
    pub fn new(output_directory: &str) -> Self {
        Self { output_directory: output_directory.to_string() }
    }

    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
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
impl NetworkServiceHandler for InsertPcapFileHandler {
    async fn handle(&self, connection_pool: Arc<Pool<Postgres>>, enveloped_request: Envelope) -> Result<Envelope, Box<dyn Error + Send + Sync>> {
        if enveloped_request.get_envelope_type() != self.get_handler_type() {
            return Err(InsertError::WrongInsertableData(
                self.get_handler_type()
                .split('-')
                .collect::<Vec<_>>()
                .join(" ")
            ).into());
        }
        let tenant_id = enveloped_request.get_tenant_id();
        let pcap_data = InsertPcapFileDTO::decode(enveloped_request.get_data());
        let packet_id = Uuid::now_v7().to_string();
        let pcap_file_path = format!("{}/{}", &self.output_directory, &packet_id);
        
        let data_packet_save_result = self.save_pcap_file_to(&pcap_file_path, pcap_data.get_data()).await;

        if let Err(e) = data_packet_save_result {
            log::error!("Error: {e}");
            return Err(InsertError::WriteFile(e.to_string()).into());
        }

        let network_packet_data = match crate::utils::decoder::Decoder::get_network_packet_data(pcap_data.get_data()).await {
            Ok(data) => data,
            Err(err_desc) => return Err(InsertError::DecodePcapFile(err_desc).into())
        };
        let mut transaction = match connection_pool.begin().await {
            Ok(transaction) => transaction,
            Err(err) => return Err(InsertError::TranscationError(err.to_string()).into()),
        };
        let insert_result = network_packet_inserter::insert_network_packet_transaction(
            &mut transaction,
            &packet_id,
            tenant_id,
            &pcap_file_path,
            &network_packet_data
        ).await; 
        match insert_result {
            Ok(_) => {
                let _ = transaction.commit().await;
                Ok(Envelope::new(tenant_id, ResultDTO::get_data_type(), &ResultDTO::new(true, None, None).encode()))
            },
            Err(e) => Err(InsertError::DbError(self.get_handler_type().to_string(), e).into()),
        }
    }

    fn get_handler_type(&self) -> String {
        InsertPcapFileDTO::get_data_type().to_string()
    }
}
