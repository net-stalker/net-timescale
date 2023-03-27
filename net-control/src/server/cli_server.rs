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
    //Fields for ServerConfig:

    // The server ID string sent at the beginning of the protocol.
    server_id: russh::SshId,
    // Authentication methods proposed to the client.
    methods: russh::MethodSet,
    // The authentication banner, usually a warning message shown to the client.
    auth_banner: Option<&'static str>,
    // Authentication rejections must happen in constant time for
    // security reasons. Russh does not handle this by default.
    auth_rejection_time: std::time::Duration,
    // Authentication rejection time override for the initial "none" auth attempt.
    // OpenSSH clients will send an initial "none" auth to probe for authentication methods.
    auth_rejection_time_initial: Option<std::time::Duration>,
    // The server's keys. The first key pair in the client's preference order will be chosen.
    keys: Vec<russh_keys::key::KeyPair>,
    // The bytes and time limits before key re-exchange.
    limits: russh::Limits,
    // The initial size of a channel (used for flow control).
    window_size: u32,
    // The maximal size of a single packet.
    maximum_packet_size: u32,
    // Internal event buffer size
    event_buffer_size: usize,
    // Lists of preferred algorithms.
    preferred: russh::Preferred,
    // Maximal number of allowed authentication attempts.
    max_auth_attempts: usize,
    // Time after which the connection is garbage-collected.
    connection_timeout: Option<std::time::Duration>,

    // Server Host
    server_host: &'static str,
    // Server Port
    server_port: &'static str,



    // Fields for ControlServer:

    // Handler (type), that will be sent to the clients and handle all the events.
    // It should be the russh::server::Handler.
    control_handler: Box<H>
}

impl <H> CLIServerBuilder <H>
where
    H: russh::server::Handler 
{
    pub fn new() -> Self {
        CLIServerBuilder {
            server_id: todo!(),
            methods: todo!(),
            auth_banner: todo!(),
            auth_rejection_time: todo!(),
            auth_rejection_time_initial: todo!(),
            keys: todo!(),
            limits: todo!(),
            window_size: todo!(),
            maximum_packet_size: todo!(),
            event_buffer_size: todo!(),
            preferred: todo!(),
            max_auth_attempts: todo!(),
            connection_timeout: todo!(),
            server_host: todo!(),
            server_port: todo!(),
            control_handler: todo!(),
        }
    }

    // pub fn build(self) -> CLIServer<H> {
    //     CLIServer {
    //     }
    // }
}