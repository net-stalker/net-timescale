use chrono::DateTime;
use chrono::Utc;

use sqlx::Transaction;
use sqlx::Postgres;
use sqlx::Pool;
use sqlx::Error;

use net_timescale_api::api::network_bandwidth::bandwidth_bucket::BandwidthBucketDTO;

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct BandwidthBucket {
    bucket: DateTime<Utc>,
    total_bytes: i64,
}

impl From<BandwidthBucket> for BandwidthBucketDTO {
    fn from(value: BandwidthBucket) -> Self {
        BandwidthBucketDTO::new(
            value.bucket.timestamp_millis(),
            value.total_bytes
        )
    }
}

const SELECT_BY_DATE_CUT: &str = "
        SELECT *
        FROM network_bandwidth_aggregate
        WHERE group_id = $1 AND bucket >= $2 AND bucket < $3;
";

impl BandwidthBucket {
    pub async fn select_by_date_cut(
        con: &Pool<Postgres>,
        group_id: Option<&str>,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<BandwidthBucket>, Error>
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
    ) -> Result<Vec<BandwidthBucket>, Error>
    {
        sqlx::query_as(SELECT_BY_DATE_CUT)
            .bind(group_id)
            .bind(start_date)
            .bind(end_date)
            .fetch_all(&mut **transaction)
            .await
    }
}
