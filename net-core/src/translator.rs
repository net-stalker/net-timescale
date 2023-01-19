pub trait Decoder {
    fn decode(data: Vec<u8>) -> String;
}