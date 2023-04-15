pub trait AsQuery {
    fn execute(&self, data: &[u8]);
}
