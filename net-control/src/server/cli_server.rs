use std::ops::Deref;

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
//TODO: Get rid of a tokio usage 
    // It is not possible. Need to find another lib
    #[tokio::main]
    pub async fn start_server(self) {
        let ip = self.config.get_server_ip();
        let port = self.config.get_server_port();

        let addrs = format!("{}:{}", ip, port);

        let arc_config = std::sync::Arc::new(self.config.get_config());
        let _run_result = russh::server::run(arc_config, addrs, self.server).await;
    }

    pub fn builder() -> CLIServerBuilder<ServerHandler> {
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
    control_handler: H
}

impl CLIServerBuilder <ServerHandler> {
    pub fn new() -> Self {
        CLIServerBuilder {
            server_id: russh::SshId::Standard(format!(
                "SSH-2.0-{}_{}",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION")
            )),
            methods: russh::MethodSet::all(),
            auth_banner: None,
            auth_rejection_time: std::time::Duration::from_secs(1),
            auth_rejection_time_initial: None,
            keys: Vec::new(),
            window_size: 2097152,
            maximum_packet_size: 32768,
            event_buffer_size: 10,
            limits: russh::Limits::default(),
            preferred: Default::default(),
            max_auth_attempts: 3,
            connection_timeout: None,

            server_host: "0.0.0.0",
            server_port: "2222",

            control_handler: ServerHandler,
        }
    }
}

impl <H> CLIServerBuilder <H>
where
    H: russh::server::Handler + Default
{
    pub fn build(self) -> CLIServer<H> {
        CLIServer {
            server: ControlServer {
                handler: self.control_handler,
            },
            
            config: ServerConfig {
                russh_config: russh::server::Config {
                    server_id: self.server_id,
                    methods: self.methods,
                    auth_banner: self.auth_banner,
                    auth_rejection_time: self.auth_rejection_time,
                    auth_rejection_time_initial: self.auth_rejection_time_initial,
                    keys: self.keys,
                    limits: self.limits,
                    window_size: self.window_size,
                    maximum_packet_size: self.maximum_packet_size,
                    event_buffer_size: self.event_buffer_size,
                    preferred: self.preferred,
                    max_auth_attempts: self.max_auth_attempts,
                    connection_timeout: self.connection_timeout,
                },

                server_ip: self.server_host,
                server_port: self.server_port,
            },
        }
    }
}