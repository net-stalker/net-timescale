use chrono::{DateTime, Utc};
use diesel::{PgConnection, QueryableByName, RunQueryDsl, sql_query};
use diesel::sql_types::{Timestamptz, Text};

#[derive(QueryableByName, Debug)]
pub struct AddressInfo {
    #[diesel(sql_type = Text)]
    pub addr: String
    // may be expandable in future
}

pub fn select_address_info_by_date_cut(con: &mut PgConnection, start_date: DateTime<Utc>, end_date: DateTime<Utc>)
                                        -> Vec<AddressInfo>
{
    let query = sql_query(
    "SELECT addr
        FROM (
            SELECT DISTINCT src_addr AS addr, frame_time
            FROM captured_traffic
            WHERE frame_time >= $1 AND frame_time <= $2
            UNION ALL
            SELECT DISTINCT dst_addr AS addr, frame_time
            FROM captured_traffic
            WHERE frame_time >= $1 AND frame_time <= $2
        ) AS subquery
        GROUP BY time_bucket('1 minute', frame_time), addr;"
    );
    query
        .bind::<Timestamptz, _>(start_date)
        .bind::<Timestamptz, _>(end_date)
        .load::<AddressInfo>(con).unwrap()
}