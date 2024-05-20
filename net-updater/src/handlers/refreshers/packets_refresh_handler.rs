use std::error::Error;
use std::sync::Arc;

use async_trait::async_trait;
use component_core::pcaps;
use component_core::pcaps::pcap_splitter::PcapSplitter;
use net_component::handler::network_service_handler::NetworkServiceHandler;
use net_core_api::api::envelope::envelope::Envelope;
use net_core_api::api::result::result::ResultDTO;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::typed_api::Typed;
use net_file::translator::pcap_translator::PcapTranslator;
use net_file::translator::translator::Translator;
use net_updater_api::api::refreshers::refresh_pcap_parsed_data::refresh_pcap_parsed_data_request::RefreshPcapParsedDataRequestDTO;
use sqlx::Pool;
use sqlx::Postgres;
use crate::core::update_error::UpdateError;
use crate::utils::networks_ids_selector;
use crate::utils::packets_by_network_id_selector;
use crate::utils::packets_parsed_data_updater;

#[derive(Default, Debug)]
pub struct PacketsRefreshHandler {}

impl PacketsRefreshHandler {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

#[async_trait]
impl NetworkServiceHandler for PacketsRefreshHandler {
    async fn handle(&self, connection_pool: Arc<Pool<Postgres>>, enveloped_request: Envelope) -> Result<Envelope, Box<dyn Error + Send + Sync>> {
        let processed_data_type = self.get_handler_type()
            .split('-')
            .collect::<Vec<_>>()
            .join(" ");
        if enveloped_request.get_envelope_type() != self.get_handler_type() {
            return Err(UpdateError::WrongUpdatableData(processed_data_type).into());
        }
        let tenant_id = enveloped_request.get_tenant_id();
        let mut transaction = match connection_pool.begin().await {
            Ok(transaction) => transaction,
            Err(err) => return Err(UpdateError::TranscationError(err.to_string()).into()),
        };
        let networks_ids = networks_ids_selector::select_networks_ids_transaction(
            &mut transaction,
            tenant_id,
        ).await?;
        for network_id in networks_ids.iter() {
            let packets = packets_by_network_id_selector::select_packets_by_network_id_transaction(
                &mut transaction,
                &network_id,
                tenant_id
            ).await?;
            let merged_pcap = pcaps::pcap_merger::PcapMerger::merge(&packets.iter().map(|pcap_info| pcap_info.pcap_file_path.as_str()).collect::<Vec<&str>>());
            let jsonb_merged_pcap = PcapTranslator::translate(merged_pcap);
            let split_pcaps_jsons: Vec<serde_json::Value> = PcapSplitter::split(&jsonb_merged_pcap)?
                .into_iter()
                .map(|jsonb_pcap| pcaps::decoder::Decoder::to_layered(jsonb_pcap))
                .take_while(|json_pcap| json_pcap.is_ok())
                .map(|json_pcap| json_pcap.unwrap())
                .collect();
            if split_pcaps_jsons.len() != packets.len() {
                return Err(UpdateError::CouldntUpdatePcaps("bad json decode during `to_layered` operation".to_string()).into());
            }
            for (pcap_info, parsed_data) in packets.iter().zip(split_pcaps_jsons.iter()) {
                if let Err(err) = packets_parsed_data_updater::update_packets_parsed_data_transaction(
                    &mut transaction,
                    pcap_info.id.as_str(),
                    parsed_data,
                    tenant_id
                ).await {
                    return Err(UpdateError::DbError(processed_data_type, err).into())
                }
            }
        }
        match transaction.commit().await {
            Ok(_) => Ok(Envelope::new(tenant_id, ResultDTO::get_data_type(), &ResultDTO::new(true, None, None).encode())),
            Err(err) => Err(UpdateError::TranscationError(err.to_string()).into()),
        }
    }

    fn get_handler_type(&self) -> String {
        RefreshPcapParsedDataRequestDTO::get_data_type().to_string()
    }
}
