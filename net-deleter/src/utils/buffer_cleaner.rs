use std::error::Error;
use sqlx::Postgres;

use crate::core::pcap_id::PcapId;

const CLEAR_QUERY_BUFFER: &str = "
    DELETE FROM Traffic_Buffer
    WHERE Tenant_Id = $1
    RETURNING Traffic_Buffer.Pcap_ID AS id;
";

pub async fn clear_buffer_transaction(
    transaction: &mut sqlx::Transaction<'_, Postgres>,
    tenant_id: &str,
) -> Result<Vec<PcapId>, Box<dyn Error + Sync + Send>> {
    let res: Vec<PcapId> = sqlx::query_as(CLEAR_QUERY_BUFFER)
        .bind(tenant_id)
        .fetch_all(&mut **transaction)
        .await?;

    Ok(res)
} 
