use russh::{self, MethodSet, Limits, Preferred};
use russh_keys::key;
use super::{server_config::{ServerConfig}, control_server::ControlServer};

pub struct CLIServer 
{
    config: ServerConfig,
    server: ControlServer,
}

impl CLIServer {
//TODO: Get rid of a tokio usage (not sure, if possible)

    #[tokio::main]
    pub async fn start_server(self) {
        let ip = self.config.get_server_ip();
        let port = self.config.get_server_port();

        let addrs = format!("{}:{}", ip, port);

        let arc_config = std::sync::Arc::new(self.config.get_config());
        let _run_result = russh::server::run(arc_config, addrs, self.server).await;
    }

    pub fn builder() -> CLIServerBuilder {
        CLIServerBuilder::new()
    }
}


pub struct CLIServerBuilder {
    config: Option<ServerConfig>,
    server: Option<ControlServer>,
}

impl CLIServerBuilder {
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

    pub fn with_server(mut self, server: ControlServer) -> Self {
        self.server = Some(server);
        self
    }


    pub fn build(self) -> CLIServer {
        CLIServer {
            config: self.config.unwrap(),
            server: self.server.unwrap()
        }
    }
}

impl Default for CLIServerBuilder {
    fn default() -> Self {
        CLIServerBuilder { 
            config: Some(ServerConfig::default()), 
            server: Some(ControlServer::new()) 
        }
    }
}