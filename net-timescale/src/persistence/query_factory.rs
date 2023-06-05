pub trait QueryFactory {
    type R;
    type Q;
    type E;
    fn create_query_handler(executor: Self::E, result_receiver: Self::R) -> Self::Q; 
}