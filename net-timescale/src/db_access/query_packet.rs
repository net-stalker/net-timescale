use std::sync::{Arc, Mutex};
use std::thread;

use log::{debug, error, info};
use postgres::fallible_iterator::FallibleIterator;
use serde_json::Value;

pub struct QueryPacket {
    pub pool: Arc<Mutex<r2d2::Pool<r2d2_postgres::PostgresConnectionManager<postgres::NoTls>>>>,
}

//FIXME Created for test. Should be refactored in the future
impl QueryPacket {
    pub fn subscribe(&self) {
        // let json_value = Self::convert_to_value(packet_json).unwrap();

        // let result = self.client.lock().unwrap()
        //     .execute(
        //         "INSERT INTO CAPTURED_TRAFFIC (frame_time, src_addr, dst_addr, binary_data) VALUES ($1, $2, $3, $4)",
        //         &[&frame_time, &src_addr, &dst_addr, &json_value],
        //     );

        // match result {
        //     Ok(_) => {}
        //     Err(error) => {
        //         error!("{}", error)
        //     }
        // }
        let arc = self.pool.clone();
        thread::spawn(move || {
            // Listen for events on channel 'myevent'.
            let mut conn = arc.lock().unwrap().get().unwrap();
            conn.execute("LISTEN core_db_event", &[]).expect("Could not send LISTEN");
            let mut notifications = conn.notifications();
            let mut it = notifications.blocking_iter();

            loop {
                info!("Waiting for notifications...");
                let a = it.next();
                match a {
                    Ok(Some(b)) => {
                        debug!("{:?}", b);
                    }
                    Err(e) => error!("Got error {:?}", e),
                    _ => panic!("Unexpected operation!!!")
                }
            }
        });
    }
    
    fn convert_to_value(packet_json: Vec<u8>) -> serde_json::Result<Value> {
        serde_json::from_slice(&*packet_json)
    }
}