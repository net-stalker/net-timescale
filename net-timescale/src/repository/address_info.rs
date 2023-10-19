use chrono::{Utc, DateTime};
use sqlx::{Error, Pool, Postgres, Transaction};


#[derive(sqlx::FromRow, Debug)]
pub struct AddressInfo {
    pub node_id: String,
    pub agent_id: String,
    // may be expandable in future
}

// TODO: reorganize such queries
// pub struct AddressInfoBuilder {
//
// }

pub const SELECT_BY_DATE_CUT: &str =
    "
            SELECT agent_id, node_id
            FROM (
                SELECT DISTINCT agent_id, src_addr AS node_id
                FROM network_graph_aggregate
                WHERE group_id = $1 AND bucket >= $2 AND bucket < $3
                UNION
                SELECT DISTINCT agent_id, dst_addr as node_id
                FROM network_graph_aggregate
                WHERE group_id = $1 AND bucket >= $2 AND bucket < $3
            ) AS info
            ORDER BY node_id;
        ";

// TODO: rewrite this stuff
impl AddressInfo {
    pub async fn select_by_date_cut(
        con: &Pool<Postgres>,
        group_id: Option<&str>,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<Self>, Error>
    {
        sqlx::query_as::<_, AddressInfo>(SELECT_BY_DATE_CUT)
            .bind(group_id)
            .bind(start_date)
            .bind(end_date)
            .fetch_all(con)
            .await
    }
    pub async fn transaction_select_by_date_cut(
        transaction: &mut Transaction<'_, Postgres>,
        group_id: Option<&str>,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<AddressInfo>, Error>
    {
        sqlx::query_as::<_, AddressInfo>(SELECT_BY_DATE_CUT)
        .bind(group_id)
        .bind(start_date)
        .bind(end_date)
        .fetch_all(&mut** transaction)
        .await
    }

}
