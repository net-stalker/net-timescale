pub struct ControlServer<H>
{
    pub(crate) handler: H
}

impl <H> ControlServer<H> 
where
    H: russh::server::Handler + Default
{
    pub fn builder() -> ControlServerBuilder<H> {
        ControlServerBuilder::new()
    }
}

impl <H : russh::server::Handler + Send + Clone> russh::server::Server for ControlServer<H> {
    type Handler = H;
//TODO: Change Handler type to a reference (Get rid of .clone())
    fn new_client(&mut self, _peer_addr: Option<std::net::SocketAddr>) -> Self::Handler {
        self.handler.clone()
    }
}

pub struct ControlServerBuilder<H>
{
    handler: Option<Box<H>>
}

impl <H> ControlServerBuilder<H> 
where
    H: russh::server::Handler + Default
{
    fn new() -> Self {
        ControlServerBuilder {
            handler: None
        }
    }

    pub fn with_handler(mut self, handler: H) -> Self {
        self.handler = Some(Box::new(handler));
        self
    }

    pub fn build(self) -> ControlServer<H> {
        ControlServer { 
            handler: H::default()
        }
    }
}