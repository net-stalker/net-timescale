use crate::command::executor::Executor;

pub trait QueryFactory {
    type R;
    type Q;
    fn create_query_handler(executor: Executor, result_receiver: Self::R) -> Self::Q; 
}