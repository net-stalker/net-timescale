use std::sync::{Arc, Mutex};
use chrono::{DateTime, Local};
pub struct SelectInterval{
    pub pool: Arc<Mutex<r2d2::Pool<r2d2_postgres::PostgresConnectionManager<postgres::NoTls>>>>
}

impl SelectInterval{
    pub fn select_packets_from_interval(&self, left_frame_time: DateTime<Local>, right_frame_time: DateTime<Local>){
        let result = self.pool.lock().unwrap()
            .get()
            .unwrap()
            .query("
                    SELECT
                        TIME_BUCKET('1 minute', \"frame_time\") AS bucket,
                        src_addr,
                        dst_addr 
                    FROM captured_traffic
                    WHERE frame_time >= $1 AND frame_time <= $2
                    GROUP BY bucket, src_addr, dst_addr;
            ", &[&left_frame_time, &right_frame_time]);
        match result {
            Ok(rows) => {
                log::info!("select_interval query result");
                for row in rows {
                    let time_bucket: DateTime<Local> = row.get("bucket");
                    let src_addr: &str = row.get("src_addr");
                    let dst_addr: &str = row.get("dst_addr"); 
                    log::info!("time_bucket: {}, src_addr: {}, dst_addr: {}", time_bucket, src_addr, dst_addr);
                } 
            },
            Err(error) => {
                log::error!("{}", error);
            }
        }
    }
}

