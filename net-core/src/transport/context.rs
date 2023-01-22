use std::sync::Arc;

pub struct Context {
    xctx: Arc<zmq::Context>,
}

impl Context {
    pub fn xctx(&self) -> &Arc<zmq::Context> {
        &self.xctx
    }
}

pub struct ContextBuilder {
    xctx: Arc<zmq::Context>,
}

impl ContextBuilder {
    pub fn new() -> ContextBuilder {
        ContextBuilder {
            xctx: Arc::new(zmq::Context::new())
        }
    }

    pub fn build(self) -> Arc<Context> {
        Arc::from(Context { xctx: self.xctx })
    }
}

mod tests {
    use super::*;

    #[test]
    fn test() {
        let context = ContextBuilder::new().build();
        let _x = context.xctx();
    }
}
