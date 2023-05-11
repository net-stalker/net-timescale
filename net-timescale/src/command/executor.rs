pub trait Executor<'a> {
    type Q;
    type E;
    type R;
    fn execute(&self, query: &'a Self::Q) -> Result<u64, Self::E>; 
    fn query(&self, query: &'a Self::Q) -> Result<Self::R, Self::E>;
}