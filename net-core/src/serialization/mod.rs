pub mod envelope;

pub trait Encoder {
    fn encode(&self) -> Vec<u8>;
}

pub trait Decoder {
    fn decode(data: Vec<u8>) -> Self;
} 