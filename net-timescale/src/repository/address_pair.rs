use chrono::{Utc, TimeZone};
use futures::stream::BoxStream;
use net_proto_api::{envelope::envelope::Envelope, decoder_api::Decoder};
use net_timescale_api::api::network_graph_request::NetworkGraphRequestDTO;
use sqlx::{Error, Pool, Postgres};

#[derive(sqlx::FromRow, Debug)]
pub struct AddressPair {
    pub src_id: String,
    pub dst_id: String,
    pub concatenated_protocols: String,
}
const PLAIN_SELECT_ADDRESS_PAIRS: &str = "
        SELECT src_addr as src_id, dst_addr as dst_id, STRING_AGG(protocols, ':' ORDER BY protocols) AS concatenated_protocols
        FROM data_aggregate
        WHERE group_id = $1 AND bucket >= $2 AND bucket < $3
        GROUP BY src_addr, dst_addr
        ORDER BY src_addr, dst_addr;
    ";
const FILTER_INCLUDE_SELECT_ADDRESS_PAIRS: &str = "
        SELECT src_addr as src_id, dst_addr as dst_id, STRING_AGG(protocols, ':' ORDER BY protocols) AS concatenated_protocols
        FROM data_aggregate
        WHERE group_id = $1 AND bucket >= $2 AND bucket < $3 AND protocols ~ {}
        GROUP BY src_addr, dst_addr
        ORDER BY src_addr, dst_addr;
    ";
const FILTER_EXCLUDE_SELECT_ADDRESS_PAIRS: &str = "
        SELECT src_addr as src_id, dst_addr as dst_id, STRING_AGG(protocols, ':' ORDER BY protocols) AS concatenated_protocols
        FROM data_aggregate
        WHERE group_id = $1 AND bucket >= $2 AND bucket < $3 AND protocols !~ {}
        GROUP BY src_addr, dst_addr
        ORDER BY src_addr, dst_addr;
    ";
pub async fn select_address_pairs_by_date_cut<'a>(
    con: &'a Pool<Postgres>,
    envelope: &'a Envelope
) -> BoxStream<'a, Result<AddressPair, Error>>
{
    let group_id = envelope.get_group_id().ok();

    let envelope_data = envelope.get_data();

    let network_graph_request = NetworkGraphRequestDTO::decode(envelope_data);
    let start_date = Utc.timestamp_millis_opt(network_graph_request.get_start_date_time()).unwrap();
    let end_date = Utc.timestamp_millis_opt(network_graph_request.get_end_date_time()).unwrap();
    let filters = network_graph_request.get_filters();
    let (query_str, regex): (&str, Option<String>) = match filters {
        Some(filters) => {
            let mut regex = "".to_string();
            filters.get_filters()
                .iter()
                .for_each(|filter| regex.push_str(format!("(?=.*(:|^){}(:|$))", filter).as_str()));

            match filters.is_include() {
                true => (FILTER_INCLUDE_SELECT_ADDRESS_PAIRS, Some(regex)),
                false => (FILTER_EXCLUDE_SELECT_ADDRESS_PAIRS, Some(regex))
            }
        },
        None => (PLAIN_SELECT_ADDRESS_PAIRS, None)
    };

    let query = sqlx::query_as::<_, AddressPair>(
        query_str
    )
        .bind(group_id)
        .bind(start_date)
        .bind(end_date);
    match regex {
        Some(regex) => query.bind(regex),
        _ => query
    }.fetch(con)
}

