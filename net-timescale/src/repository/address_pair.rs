use chrono::{Utc, DateTime};
use sqlx::{Error, Pool, Postgres, Transaction};

#[derive(sqlx::FromRow, Debug)]
pub struct AddressPair {
    pub src_id: String,
    pub dst_id: String,
    pub concatenated_protocols: String,
}
const SELECT_BY_DATE_CUT: &str = "
            SELECT src_addr as src_id, dst_addr as dst_id, STRING_AGG(protocols, ':' ORDER BY protocols) AS concatenated_protocols
            FROM network_graph_aggregate
            WHERE group_id = $1 AND bucket >= $2 AND bucket < $3
            GROUP BY src_addr, dst_addr
            ORDER BY src_addr, dst_addr;
        ";
impl AddressPair {
    pub async fn select_by_date_cut(
        con: &Pool<Postgres>,
        group_id: Option<&str>,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<AddressPair>, Error>
    {
        sqlx::query_as::<_, AddressPair>(SELECT_BY_DATE_CUT)
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
    ) -> Result<Vec<AddressPair>, Error>
    {
        sqlx::query_as::<_, AddressPair>(SELECT_BY_DATE_CUT)
            .bind(group_id)
            .bind(start_date)
            .bind(end_date)
            .fetch_all(&mut **transaction)
            .await
    }
}

