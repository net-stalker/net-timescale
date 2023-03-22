use std::collections::HashSet;

pub struct ConnectionPool{
    pool: elephantry::Pool,
    con_names: HashSet<u32>
}

impl ConnectionPool {
    pub fn new(connection_string: &str, amount_of_connections: u32) -> ConnectionPool{
        let mut pool = elephantry::Pool::new(connection_string)
                        .unwrap();
        let mut con_names = HashSet::<u32>::new();
        
        for i in 0..amount_of_connections{
            pool = pool.add_connection(i.to_string().as_str(), connection_string).unwrap();
            con_names.insert(i);
        }
        ConnectionPool {
            pool,
            con_names
        }
    }
    // get_connection - wait until there will be a free connection or return it immidiately
}