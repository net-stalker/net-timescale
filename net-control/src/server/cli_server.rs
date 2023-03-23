use russh;
use super::{server_config::{ServerConfig}, control_server::ControlServer, server_handler::ServerHandler};

pub struct CLIServer <H>
where
    H: russh::server::Handler
{
    config: ServerConfig,
    server: ControlServer<H>,
}

impl <H> CLIServer<H> 
where
    H: russh::server::Handler + Send + Clone + 'static
{
//TODO: Get rid of a tokio usage (not sure, if possible)

    #[tokio::main]
    pub async fn start_server(self) {
        let ip = self.config.get_server_ip();
        let port = self.config.get_server_port();

        let addrs = format!("{}:{}", ip, port);

        let arc_config = std::sync::Arc::new(self.config.get_config());
        let _run_result = russh::server::run(arc_config, addrs, self.server).await;
    }

    pub fn builder() -> CLIServerBuilder<H> {
        CLIServerBuilder::new()
    }
}


pub struct CLIServerBuilder <H> 
where
    H: russh::server::Handler
{
    config: Option<ServerConfig>,
    server: Option<ControlServer<H>>,
}

impl <H> CLIServerBuilder <H>
where
    H: russh::server::Handler 
{
    pub fn new() -> Self {
        CLIServerBuilder { 
            config: None, 
            server: None 
        }
    }

    pub fn with_config(mut self, config: ServerConfig) -> Self {
        self.config = Some(config);
        self
    }

    pub fn with_server(mut self, server: ControlServer<H>) -> Self {
        self.server = Some(server);
        self
    }


    pub fn build(self) -> CLIServer<H> {
        CLIServer {
            config: self.config.unwrap(),
            server: self.server.unwrap()
        }
    }
}