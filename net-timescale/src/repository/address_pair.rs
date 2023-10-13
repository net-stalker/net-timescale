use chrono::{Utc, TimeZone};
use futures::stream::BoxStream;
use net_proto_api::{envelope::envelope::Envelope, decoder_api::Decoder};
use net_timescale_api::api::network_graph_request::NetworkGraphRequestDTO;
use sqlx::{Error, Pool, Postgres};

use crate::persistence::network_graph::NetworkGraphRequest;

#[derive(sqlx::FromRow, Debug)]
pub struct AddressPair {
    pub src_id: String,
    pub dst_id: String,
    pub concatenated_protocols: String,
}

pub async fn select_address_pairs_by_date_cut<'a>(
    con: &'a Pool<Postgres>,
    envelope: &'a Envelope
) -> BoxStream<'a, Result<AddressPair, Error>>
{
    let group_id = envelope.get_group_id().ok();

    let envelope_data = envelope.get_data();

    let network_graph_request: NetworkGraphRequest = NetworkGraphRequestDTO::decode(envelope_data).into();
    let start_date = Utc.timestamp_millis_opt(network_graph_request.get_start_date_time()).unwrap();
    let end_date = Utc.timestamp_millis_opt(network_graph_request.get_end_date_time()).unwrap();

    sqlx::query_as::<_, AddressPair>(
        "
            SELECT src_addr as src_id, dst_addr as dst_id, STRING_AGG(protocols, ':' ORDER BY protocols) AS concatenated_protocols
            FROM data_aggregate
            WHERE group_id = $1 AND bucket >= $2 AND bucket < $3
            GROUP BY src_addr, dst_addr
            ORDER BY src_addr, dst_addr;
        "
    )
        .bind(group_id)
        .bind(start_date)
        .bind(end_date)
        .fetch(con)
}
