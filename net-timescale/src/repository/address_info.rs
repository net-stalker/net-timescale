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
        // TODO: this query isn't very efficient because we have to do 2 sub-queries.
    "SELECT distinct src_addr as addr
        FROM captured_traffic
        WHERE frame_time >= $1 AND frame_time <= $2
        union
        SELECT distinct dst_addr as addr
        FROM captured_traffic
        WHERE frame_time >= $1 AND frame_time <= $2;"
    );
    query
        .bind::<Timestamptz, _>(start_date)
        .bind::<Timestamptz, _>(end_date)
        .load::<AddressInfo>(con).unwrap()
}