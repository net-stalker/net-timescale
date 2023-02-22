use russh::{self, Channel, server::{Msg, Session}};


pub struct CLIServer 
{
    config: ServerConfig,
    server: ControlServer,
}

impl CLIServer {
    pub fn new() -> Self {
        CLIServer { config: ServerConfig::new(), server: ControlServer::new() }
    }

    #[tokio::main]
    pub async fn start_server(self) {
        let arc_config = std::sync::Arc::new(self.config.russh_config);
        let _run_result = russh::server::run(arc_config, "0.0.0.0:2222", self.server).await;
    }
}

struct ServerConfig {
    russh_config: russh::server::Config
}

impl ServerConfig {
    fn new() -> Self {
        ServerConfig { russh_config: russh::server::Config::default() }
    }
}

struct ControlServer {
    handler: ServerHandler
}

impl ControlServer {
    fn new() -> Self {
        ControlServer { handler: ServerHandler::new() }
    }
}

impl russh::server::Server for ControlServer {
    type Handler = ServerHandler;

    fn new_client(&mut self, _peer_addr: Option<std::net::SocketAddr>) -> Self::Handler {
        self.handler.clone()
    }
}

#[derive(Clone)]
struct ServerHandler {}

impl ServerHandler {
    fn new() -> Self {
        ServerHandler {}
    }
}

#[async_trait::async_trait]
impl russh::server::Handler for ServerHandler {
    type Error = anyhow::Error;

    async fn channel_open_session(self, _channel: Channel<Msg>, session: Session) -> Result<(Self, bool, Session), Self::Error> {
        Ok((self, true, session))
    }
}