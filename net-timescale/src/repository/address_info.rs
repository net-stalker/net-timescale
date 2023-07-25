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
    "
            SELECT addr
            FROM (
                SELECT DISTINCT src_addr AS addr
                FROM address_pair_aggregate
                WHERE bucket >= $1 AND bucket < $2
                UNION
                SELECT distinct dst_addr as addr
                FROM address_pair_aggregate
                WHERE bucket >= $1 AND bucket < $2
            ) AS info
            ORDER BY addr;
        "
    );
    query
        .bind::<Timestamptz, _>(start_date)
        .bind::<Timestamptz, _>(end_date)
        .load::<AddressInfo>(con).unwrap()
}