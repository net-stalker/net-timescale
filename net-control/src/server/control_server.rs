use super::server_handler::ServerHandler;

pub struct ControlServer {
    handler: ServerHandler
}

impl ControlServer {
    pub (super) fn new() -> Self {
        ControlServer { handler: ServerHandler::new() }
    }   
}

impl russh::server::Server for ControlServer {
    type Handler = ServerHandler;
//TODO: Change Handler type to a reference (Get rid of .clone())
    fn new_client(&mut self, _peer_addr: Option<std::net::SocketAddr>) -> Self::Handler {
        self.handler.add_new_client_to_the_aggregator();

        self.handler.clone()
    }
}