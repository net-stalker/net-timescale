pub trait Decoder {
    type Input;
    type Output;

    fn decode(data: Self::Input) -> Self::Output;
}