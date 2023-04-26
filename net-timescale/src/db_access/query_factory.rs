use crate::command::executor::Executor;

pub trait QueryFactory {
    type Q;
    fn create_query_handler(executor: Executor, sender_endpoint: &str) -> Self::Q; 
}