use sqlx::{Pool, Postgres};
use net_timescale::repository::continuous_aggregate;

pub use net_timescale::repository::continuous_aggregate::drop_data_aggregate;
pub async fn create_data_aggregate(con: &Pool<Postgres>) {
    continuous_aggregate::create_data_aggregate(con).await.unwrap();
    continuous_aggregate::add_refresh_policy_for_data_aggregate(con).await.unwrap();
}
