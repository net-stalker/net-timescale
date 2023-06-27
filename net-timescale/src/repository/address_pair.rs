use chrono::{DateTime, Utc};
use diesel::{PgConnection, QueryableByName, RunQueryDsl, sql_query};
use diesel::sql_types::{Timestamptz, Text};

#[derive(QueryableByName, Debug)]
pub struct AddressPair {
    #[diesel(sql_type = Text)]
    pub src_addr: String,
    #[diesel(sql_type = Text)]
    pub dst_addr: String,
}

pub fn select_address_pairs_by_date_cut(con: &mut PgConnection, start_date: DateTime<Utc>, end_date: DateTime<Utc>)
                                        -> Vec<AddressPair>
{
    let query = sql_query(
        "SELECT \
            src_addr,
            dst_addr
        FROM captured_traffic
        WHERE frame_time >= $1 AND frame_time <= $2
        GROUP BY src_addr, dst_addr;"
    );
    query
        .bind::<Timestamptz, _>(start_date)
        .bind::<Timestamptz, _>(end_date)
        .load::<AddressPair>(con).unwrap()
}
