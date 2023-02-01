use std::sync::Arc;

pub struct Context;

impl Context {}

pub struct ContextBuilder {}

impl ContextBuilder {
    pub fn new() -> Self {
        ContextBuilder{}
    }
    pub fn build(self) -> Arc<Context> {
        Arc::from(Context)
    }
}

mod tests {
    use super::*;

    #[test]
    fn test() {
        let _context = ContextBuilder::new().build();
    }
}
