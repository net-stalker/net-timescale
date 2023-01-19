pub trait Reader {
    fn read(path: &str) -> Vec<u8>;
}
