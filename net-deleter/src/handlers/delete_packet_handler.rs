use std::error::Error;
use std::sync::Arc;

use net_primitives::api::integer::Integer;
use tokio::fs;
use net_component::handler::network_service_handler::NetworkServiceHandler;
use net_core_api::api::envelope::envelope::Envelope;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::typed_api::Typed;
use net_deleter_api::api::packets::DeletePacketsRequestDTO;
use sqlx::Pool;
use sqlx::Postgres;

use crate::core::delete_error::DeleteError;
use crate::utils::network_packets_deleter;

#[derive(Default, Debug)]
pub struct DeleteNetworkPacketHandler {
    pcap_files_directory: String,
}

impl DeleteNetworkPacketHandler {
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
impl NetworkServiceHandler for DeleteNetworkPacketHandler {
    // need to trigger refreshes
    async fn handle(&self, connection_pool: Arc<Pool<Postgres>>, enveloped_request: Envelope) -> Result<Envelope, Box<dyn Error + Send + Sync>> {
        let deletable_data_type = self.get_handler_type().split('-').collect::<Vec<_>>().join(" ");
        if enveloped_request.get_envelope_type() != self.get_handler_type() {
            return Err(DeleteError::WrongDeletableData(
                deletable_data_type
            ).into());
        }
        let tenant_id = enveloped_request.get_tenant_id();
        let packets_to_delete = DeletePacketsRequestDTO::decode(enveloped_request.get_data());
        let mut transaction = match connection_pool.begin().await {
            Ok(transaction) => transaction,
            Err(err) => return Err(DeleteError::TranscationError(err.to_string()).into()),
        };
        // Delete from main table
        let mapped_packets_to_delete = packets_to_delete.get_ids().iter().map(|id| id.as_str()).collect::<Vec<&str>>();
        let delete_packets_res = network_packets_deleter::delete_network_packets_transaction(
            &mut transaction,
            &mapped_packets_to_delete,
            tenant_id,
        ).await;
        if let Err(err) = delete_packets_res {
            return Err(DeleteError::DbError(deletable_data_type, err).into());
        }
        // Delete from buffer table
        let delete_packets_res = network_packets_deleter::delete_network_packets_buffer_transaction(
            &mut transaction,
            &mapped_packets_to_delete,
            tenant_id,
        ).await;
        match delete_packets_res {
            Ok(updated_rows) => {
                let _ = transaction.commit().await;
                for id in packets_to_delete.get_ids() {
                    // looks like we don't really bother about the result of delete operation
                    let _ = self.delete_pcap_file(id.as_str()).await;
                }
                Ok(Envelope::new(tenant_id, Integer::get_data_type(), &Integer::new(updated_rows.rows_affected() as i64).encode()))
            },
            Err(err) => Err(DeleteError::DbError(deletable_data_type, err).into()),
        }
    }

    fn get_handler_type(&self) -> String {
        DeletePacketsRequestDTO::get_data_type().to_string()
    }
}
