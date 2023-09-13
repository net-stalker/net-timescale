use capnpc::schema_capnp::field::group;
use chrono::{DateTime, Utc, TimeZone};
use net_proto_api::{envelope::envelope::Envelope, decoder_api::Decoder};
use net_timescale_api::api::network_graph_request::{self, NetworkGraphRequestDTO};
use sqlx::{Error, Pool, Postgres};
use futures::stream::BoxStream;

use crate::persistence::network_graph::NetworkGraphRequest;


#[derive(sqlx::FromRow, Debug)]
pub struct AddressInfo {
    pub id: String,
    pub aggregator: String,
    // may be expandable in future
}

pub async fn select_address_info_by_date_cut<'a>(
    con: &'a Pool<Postgres>,
    envelope: &'a Envelope
) -> BoxStream<'a, Result<AddressInfo, Error>>
{
    // TODO: this query isn't very efficient because we have to do 2 sub-queries.
    let group_id = envelope.get_group_id().ok();
    let envelope_data = envelope.get_data();

    let network_graph_request: NetworkGraphRequest = NetworkGraphRequestDTO::decode(envelope_data).into();
    
    let start_date = Utc.timestamp_millis_opt(network_graph_request.get_start_date_time()).unwrap();
    let end_date = Utc.timestamp_millis_opt(network_graph_request.get_end_date_time()).unwrap();
    sqlx::query_as::<_, AddressInfo>(
        "
            SELECT group_id, addr
            FROM (
                SELECT DISTINCT group_id, src_addr AS addr
                FROM address_pair_aggregate
                WHERE group_id = $1 AND bucket >= $2 AND bucket < $3
                UNION
                SELECT DISTINCT group_id, dst_addr as addr
                FROM address_pair_aggregate
                WHERE group_id = $1 AND bucket >= $2 AND bucket < $3
            ) AS info
            ORDER BY addr;
        "
    )
        .bind(group_id)
        .bind(start_date)
        .bind(end_date)
        .fetch(con)
}