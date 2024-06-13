use std::error::Error;
use std::sync::Arc;

use net_core_api::api::primitives::none::None;
use net_deleter_api::api::buffered_packet::DeleteBufferedPacketRequestDTO;
use tokio::fs;
use net_component::handler::network_service_handler::NetworkServiceHandler;
use net_core_api::api::envelope::envelope::Envelope;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::typed_api::Typed;
use sqlx::Pool;
use sqlx::Postgres;

use crate::core::delete_error::DeleteError;
use crate::utils::buffered_network_packet_deleter;

#[derive(Debug)]
pub struct DeleteBufferedNetworkPacketHandler {
    pcap_files_directory: String,
}

impl DeleteBufferedNetworkPacketHandler {
    pub fn new(pcap_files_directory: &str) -> Self {
        Self { pcap_files_directory: pcap_files_directory.to_string() }
    }

    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    async fn delete_pcap_file(&self, pcap_file_name: &str) -> Result<(), DeleteError> {
        let path_to_file = format!("{}/{}", self.pcap_files_directory, pcap_file_name);
        match fs::remove_file(path_to_file).await {
            Ok(()) => Ok(()),
            Err(err) => Err(DeleteError::DeleteFile(err.to_string()))
        }
    }
}

#[async_trait::async_trait]
impl NetworkServiceHandler for DeleteBufferedNetworkPacketHandler {
    async fn handle(&self, connection_pool: Arc<Pool<Postgres>>, enveloped_request: Envelope) -> Result<Envelope, Box<dyn Error + Send + Sync>> {
        let deletable_data_type = self.get_handler_type().split('-').collect::<Vec<_>>().join(" ");
        if enveloped_request.get_envelope_type() != self.get_handler_type() {
            return Err(DeleteError::WrongDeletableData(
                deletable_data_type
            ).into());
        }
        let tenant_id = enveloped_request.get_tenant_id();
        let packet_to_delete = DeleteBufferedPacketRequestDTO::decode(enveloped_request.get_data());
        let mut transaction = match connection_pool.begin().await {
            Ok(transaction) => transaction,
            Err(err) => return Err(DeleteError::TranscationErrorStart(err.to_string()).into()),
        };
        let delete_packets_res = buffered_network_packet_deleter::delete_network_packet_buffer_transaction(
            &mut transaction,
            packet_to_delete.get_id(),
            tenant_id,
        ).await;
        if let Err(err) = delete_packets_res {
            return Err(DeleteError::DbError(deletable_data_type, err).into());
        }
        if let Err(err) = transaction.commit().await {
            return Err(DeleteError::TranscationErrorEnd(err.to_string()).into());
        }
        self.delete_pcap_file(packet_to_delete.get_id()).await.unwrap_or_else(|_| log::debug!("Couldn't delete the file"));
        Ok(Envelope::new(tenant_id, None::get_data_type(), &None::default().encode()))
    }

    fn get_handler_type(&self) -> String {
        DeleteBufferedPacketRequestDTO::get_data_type().to_string()
    }
}
