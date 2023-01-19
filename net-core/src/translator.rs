pub trait Decoder {
    fn decode(path: Vec<u8>) -> String;
}