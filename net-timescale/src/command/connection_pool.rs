use std::collections::HashSet;
use std::sync::RwLock;
use std::sync::Arc;
use std::sync::atomic::AtomicU32;
use queues::*;

pub struct ConnectionPool{
    pool: Arc<RwLock<elephantry::Pool>>,
    con_queue: Arc<RwLock<Queue<u32>>>,
    next_id: AtomicU32
}
pub struct PoolConnection<'a>{
    // probably there is no need to sync the con attribute
    father: &'a ConnectionPool,
    pub con: Arc<RwLock<elephantry::Connection>>,
    pub con_id: u32 
}

impl<'a> Drop for PoolConnection<'a>{
    fn drop(&mut self){
        self.father.set_free_connection(self);
    }
}

impl ConnectionPool {
    pub fn new(connection_string: &str, amount_of_connections: u32) -> elephantry::Result<ConnectionPool>{
        let mut pool = elephantry::Pool::new(connection_string)?;
        let mut con_queue = Queue::<u32>::new();
        let mut con_ids = HashSet::<u32>::new();
        for i in 0..amount_of_connections{
            pool = pool.add_connection(i.to_string().as_str(), connection_string)?;
            con_queue.add(i).unwrap();
            con_ids.insert(i);
        }
        Ok(ConnectionPool {
            pool: Arc::new(RwLock::new(pool)),
            con_queue: Arc::new(RwLock::new(con_queue)),
            next_id: AtomicU32::new(amount_of_connections)
        })
    }
    pub fn get_free_connection(&self) -> Result<PoolConnection, &'static str>{
        // TODO: add time_to_wait parameter in the method.
        // after time_to_wait an error will be returned
        if let Ok(id) = self.con_queue.write().unwrap().remove(){
            if let Ok(free_pool) = self.pool.read(){
                return Ok(
                    PoolConnection{
                        father: self,
                        con: Arc::new(RwLock::new(free_pool.get(id.to_string().as_str()).unwrap().clone())),
                        con_id: id
                    }
                );
            }
            self.con_queue.write().unwrap().add(id).unwrap();
        }

        Err("No free connections")
    }
    pub fn add_connection(&mut self, connection_string: &str) -> bool {
        match self.pool.write() {
            Ok(mut con_pool) => {
                let temp_next_id = *self.next_id.get_mut();
                let new_pool = con_pool.clone();
                *con_pool = new_pool.add_connection(temp_next_id.to_string().as_str(), connection_string).unwrap();
            
                self.con_queue.write().unwrap().add(temp_next_id).unwrap();
                *self.next_id.get_mut() += 1;

                return true;
            },
            Err(_) => return false
        }
    }
    pub fn get_size(&self) -> usize{
        self.con_queue.read().unwrap().size()
    }
    fn set_free_connection(&self, connection: &PoolConnection) {
        self.con_queue.write().unwrap().add(connection.con_id).unwrap();
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    // To check if the connections are being set 
    // use Wireshark with filter 'tcp.port == 5432 && tcp.flags.syn == 1 && tcp.flags.ack == 0'
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
        {
            let free_con = pool.get_free_connection().unwrap();
            assert_eq!(pool.get_size(), 1);
        }

        assert_eq!(pool.get_size(), 2);
    }
    #[test]
    fn add_connection_test(){
        let pool_res = ConnectionPool::new("postgres://postgres:PsWDgxZb@localhost", 2);
        assert!(pool_res.is_ok(), "Connection has failed. Check if connection_string is correct or if DB is set up");
        let mut pool = pool_res.unwrap();
        
        assert_eq!(pool.add_connection("postgres://postgres:PsWDgxZb@localhost"), true);

        assert_eq!(pool.get_size(), 3);
    }

}