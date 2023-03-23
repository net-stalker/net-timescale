use std::collections::HashSet;
use std::sync::RwLock;
use std::sync::Arc;

pub struct ConnectionPool{
    pool: Arc<RwLock<elephantry::Pool>>,
    con_ids: Arc<RwLock<HashSet<u32>>>
}
pub struct PoolConnection{
    pub con: elephantry::Connection,
    pub con_id: u32 
} 

impl ConnectionPool {
    pub fn new(connection_string: &str, amount_of_connections: u32) -> elephantry::Result<ConnectionPool>{
        let mut pool = elephantry::Pool::new(connection_string)?;
        let mut con_ids = HashSet::<u32>::new();
        
        for i in 0..amount_of_connections{
            pool = pool.add_connection(i.to_string().as_str(), connection_string)?;
            con_ids.insert(i);
        }
        Ok(ConnectionPool {
            pool: Arc::new(RwLock::new(pool)),
            con_ids: Arc::new(RwLock::new(con_ids))
        })
    }
    pub fn get_free_connection(&self) -> Result<PoolConnection, &'static str>{
        todo!("Implement get_free_connection");
    }
    pub fn set_free_connection(&self, connection: PoolConnection) -> Result<(), &'static str>{
        // I can possibly push such a connection which isn't from that connection_pool
        // So I have to return something 
        todo!("Implement set_free_connection"); 
    }
    pub fn add_connection(self, con_id: u32) -> elephantry::Result<ConnectionPool>{
        todo!("Implement add_connection"); 
    }
    pub fn get_size(&self) -> usize{
        self.con_ids.read().unwrap().len()
    }
    // get_connection - wait until there will be a free connection or return it immidiately
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn new_test(){
        let pool_res = ConnectionPool::new("postgres://postgres:PsWDgxZb@localhost", 3);
        assert!(pool_res.is_ok(), "Connection has failed. Check if connection_string is correct or if DB is set up");
        let pool = pool_res.unwrap();
        assert_eq!(pool.get_size(), 3);
    }
    #[test]
    fn get_free_connection_test(){
        let pool_res = ConnectionPool::new("postgres://postgres:PsWDgxZb@localhost", 2);
        assert!(pool_res.is_ok(), "Connection has failed. Check if connection_string is correct or if DB is set up");
        let pool = pool_res.unwrap();

        let free_con_res = pool.get_free_connection();
        assert!(free_con_res.is_ok(), "Something went wrong. No free connections available");
        let free_con = free_con_res.unwrap();
        assert_eq!(free_con.con_id, 0);
        
        assert_eq!(pool.get_size(), 1);
    }
    #[test]
    fn set_free_connection_test(){
        let pool_res = ConnectionPool::new("postgres://postgres:PsWDgxZb@localhost", 2);
        assert!(pool_res.is_ok(), "Connection has failed. Check if connection_string is correct or if DB is set up");
        let pool = pool_res.unwrap();

        let free_con = pool.get_free_connection().unwrap();
        assert_eq!(pool.get_size(), 1);

        assert!(pool.set_free_connection(free_con).is_ok(), "Set hasn't been a success");

        assert_eq!(pool.get_size(), 2);
    }
    #[test]
    fn add_connection_test(){
        let pool_res = ConnectionPool::new("postgres://postgres:PsWDgxZb@localhost", 2);
        assert!(pool_res.is_ok(), "Connection has failed. Check if connection_string is correct or if DB is set up");
        let pool = pool_res.unwrap();

        let pool_res = pool.add_connection(2);
        assert!(pool_res.is_ok(), "Something went wrong while adding a new connection");
        let pool = pool_res.unwrap();

        assert_eq!(pool.get_size(), 3);
    }

}