pub trait FileReader {
    fn read(path: &str) -> Vec<u8>;
}
