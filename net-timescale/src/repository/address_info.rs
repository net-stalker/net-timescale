use chrono::{Utc, TimeZone};
use net_proto_api::{envelope::envelope::Envelope, decoder_api::Decoder};
use net_timescale_api::api::network_graph_request::{NetworkGraphRequestDTO};
use sqlx::{Error, Pool, Postgres};
use futures::stream::BoxStream;

use crate::persistence::network_graph::NetworkGraphRequest;


#[derive(sqlx::FromRow, Debug)]
pub struct AddressInfo {
    pub node_id: String,
    pub agent_id: String,
    // may be expandable in future
}

pub async fn select_address_info_by_date_cut<'a>(
    con: &'a Pool<Postgres>,
    envelope: &'a Envelope
) -> BoxStream<'a, Result<AddressInfo, Error>>
{
    let group_id = envelope.get_group_id().ok();
    let envelope_data = envelope.get_data();

    let network_graph_request: NetworkGraphRequest = NetworkGraphRequestDTO::decode(envelope_data).into();
    
    let start_date = Utc.timestamp_millis_opt(network_graph_request.get_start_date_time()).unwrap();
    let end_date = Utc.timestamp_millis_opt(network_graph_request.get_end_date_time()).unwrap();
    sqlx::query_as::<_, AddressInfo>(
        "
            SELECT agent_id, node_id
            FROM (
                SELECT DISTINCT agent_id, src_addr AS node_id
                FROM data_aggregate
                WHERE group_id = $1 AND bucket >= $2 AND bucket < $3
                UNION
                SELECT DISTINCT agent_id, dst_addr as node_id
                FROM data_aggregate
                WHERE group_id = $1 AND bucket >= $2 AND bucket < $3
            ) AS info
            ORDER BY node_id;
        "
    )
        .bind(group_id)
        .bind(start_date)
        .bind(end_date)
        .fetch(con)
}
