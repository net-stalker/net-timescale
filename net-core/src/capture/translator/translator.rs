pub trait Translator {
    type Input;
    type Output;

    fn translate(data: Self::Input) -> Self::Output;
}