use chrono::{DateTime, Utc};
use postgres::types::ToSql;
use crate::persistence::postgres_query;

pub struct NetworkPacketQuery<'a> {
    pub raw_query: &'a str,
    pub args: [&'a (dyn ToSql + Sync); 4]
}

impl<'a> NetworkPacketQuery<'a> {
    pub fn new(time: &'a DateTime<Utc>, src_addr: &'a String, dst_addr: &'a String, json_data: &'a serde_json::Value) -> Self {
        NetworkPacketQuery { 
            raw_query: "INSERT INTO CAPTURED_TRAFFIC (frame_time, src_addr, dst_addr, binary_data) VALUES ($1, $2, $3, $4)",
            args: [
                time,
                src_addr,
                dst_addr,
                json_data
            ]
        } 
    }
}

impl<'a> postgres_query::PostgresQuery<'a> for NetworkPacketQuery<'a> {
    fn get_query_params(&self) -> (&'a str, &[&'a(dyn postgres::types::ToSql + Sync)]) {
        (self.raw_query, &self.args)
    }
}


#[cfg(test)]
mod tests{
    use crate::persistence::postgres_query::PostgresQuery;

    use super::*;
    #[test]
    fn test_add_packet_query_raw_params(){
        let time_to_insert = "2020-01-01 00:00:00.000 UTC".parse::<chrono::DateTime<chrono::Utc>>().unwrap();
        let src = "1".to_owned();
        let dst = "2".to_owned();
        let data = r#"{"test":"test"}"#;
        let json_data: serde_json::Value = serde_json::from_str(data).unwrap();
        let query_struct = NetworkPacketQuery::new(&time_to_insert, &src, &dst, &json_data);
        
        let (query, params) = query_struct.get_query_params();
        assert_eq!(query, "INSERT INTO CAPTURED_TRAFFIC (frame_time, src_addr, dst_addr, binary_data) VALUES ($1, $2, $3, $4)");
        
        let test_params: [&(dyn ToSql + Sync); 4] = [&time_to_insert, &src, &dst, &json_data];
        assert_eq!(format!("{:?}", params), format!("{:?}", &test_params));
    }
}