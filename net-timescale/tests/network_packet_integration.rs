#[cfg(feature = "integration")]
mod tests {
    use async_std::task::block_on;

    use chrono::TimeZone;
    use chrono::Utc;

    use sqlx::Pool;
    use sqlx::Postgres;

    use serde_json::json;

    use net_timescale::repository::network_packet;
    use net_timescale_api::api::network_packet::network_packet::NetworkPacketDTO;


    async fn establish_connection() -> Pool<Postgres> {
        let database_url = "postgres://postgres:PsWDgxZb@localhost".to_owned();
        Pool::<Postgres>::connect("postgres://postgres:PsWDgxZb@localhost").await
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    }

    #[test]
    fn integration_test_insert() {
        #[derive(sqlx::FromRow, Debug)]
        struct Record {
            pub frame_time: DateTime<Utc>,
            pub group_id: String,
            pub agent_id: String,
            pub src_addr: String,
            pub dst_addr: String,
            pub binary_data: serde_json::Value,
        }

        use chrono::DateTime;
        use net_proto_api::{envelope::envelope::Envelope, encoder_api::Encoder};

        let con = block_on(establish_connection());
        let json_data = json!({
            "test": "test",
        });
        let binary_json = serde_json::to_vec(&json_data).unwrap();
        let timestamp = 1688714981480935000;
        let network_packet_dto = NetworkPacketDTO::new(
            timestamp,
            "src",
            "dst",
            &binary_json,
        );
        let group_id = "some group";
        let agent_id = "some agent";
        let mut transcation = block_on(con.begin()).unwrap();
        let result = block_on(network_packet::insert_network_packet_transaction(
            &mut transcation, Envelope::new(Some(group_id), Some(agent_id), "network_packet", &network_packet_dto.encode())
        )).unwrap();
        assert_eq!(1, result.rows_affected());
        let test_query = "
            select * from captured_traffic
            where agent_id = 'some agent';
        ";
        let query = sqlx::query_as::<_, Record>(test_query).fetch_all(&mut *transcation);
        let query_result = block_on(query).unwrap();
        assert_eq!(query_result.len(), 1);
        let query_result = query_result.first().unwrap();
        assert_eq!(query_result.frame_time, Utc.timestamp_nanos(timestamp));
        assert_eq!(query_result.group_id, group_id);
        assert_eq!(query_result.agent_id, agent_id);
        assert_eq!(query_result.src_addr, "src");
        assert_eq!(query_result.src_addr, "src");
        assert_eq!(query_result.dst_addr, "dst");
        assert_eq!(query_result.binary_data, json_data);
    }
}