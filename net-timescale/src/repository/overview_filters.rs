use chrono::DateTime;
use chrono::Utc;

use sqlx::Transaction;
use sqlx::Postgres;
use sqlx::Pool;
use sqlx::Error;

use net_timescale_api::api::overview_dashboard_filters::filter_entry::FilterEntryDTO;

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct OverviewFilters {
    pub endpoint: String,
    pub protocols: Vec<String>,
    pub bytes_rec: i64,
    pub bytes_sent: i64,
}

impl From<OverviewFilters> for FilterEntryDTO {
    fn from(value: OverviewFilters) -> Self {
        FilterEntryDTO::new(
            value.endpoint.as_str(),
            &value.protocols,
            value.bytes_rec,
            value.bytes_sent,
        )
    }
}

const SELECT_BY_DATE_CUT: &str = "
    select
    COALESCE(lhs.id, rhs.id) as endpoint,
    ARRAY_REMOVE(ARRAY(SELECT DISTINCT unnest(string_to_array(COALESCE(lhs.concatenated_protocols, '') || ':' || COALESCE(rhs.concatenated_protocols, ''), ':'))), '') AS protocols,
    COALESCE(rhs.bytes_received, 0) as bytes_rec,
    COALESCE(lhs.bytes_sent, 0) as bytes_sent
    from
    (
        select
            src_addr as id,
            SUM(packet_length) as bytes_sent,
            STRING_AGG(protocols, ':' ORDER BY protocols) AS concatenated_protocols
        from overview_dashboard_filters
        where group_id = $1 AND bucket >= $2 AND bucket < $3
        group by src_addr
    ) as lhs full outer join (
        select
            dst_addr as id,
            SUM(packet_length) as bytes_received,
            STRING_AGG(protocols, ':' ORDER BY protocols) AS concatenated_protocols
        from overview_dashboard_filters
        where group_id = $1 AND bucket >= $2 AND bucket < $3
        group by dst_addr
    ) as rhs on lhs.id = rhs.id;
";

impl OverviewFilters {
    pub async fn select_by_date_cut(
        con: &Pool<Postgres>,
        group_id: Option<&str>,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<OverviewFilters>, Error>
    {
        sqlx::query_as(SELECT_BY_DATE_CUT)
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
    ) -> Result<Vec<OverviewFilters>, Error>
    {
        sqlx::query_as(SELECT_BY_DATE_CUT)
            .bind(group_id)
            .bind(start_date)
            .bind(end_date)
            .fetch_all(&mut **transaction)
            .await
    }
}
