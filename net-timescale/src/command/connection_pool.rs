

pub struct ConnectionPool{
    pub pool: elephantry::Pool
}

impl ConnectionPool {
    pub fn new(connection_string: &str, amount_of_connections: u32) -> ConnectionPool{
        // Here is an initial connection is done
        // So I have to do amount_of_connectins connection
        let mut pool = elephantry::Pool::new(connection_string)
                        .unwrap();
        
        for i in 0..amount_of_connections{
            pool = pool.add_connection(i.to_string().as_str(), connection_string).unwrap();
        }
        // So, I have to see amount_of_connections in wireshark
        ConnectionPool {
            pool: pool
        }
    }
}