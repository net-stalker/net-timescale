use russh::{self, Channel, server::{Msg, Session, Auth}, MethodSet, ChannelId, CryptoVec, Limits, Preferred};
use russh_keys::key;
use super::server_config::{ServerConfig};

pub struct CLIServer 
{
    config: ServerConfig,
    server: ControlServer,
}

impl CLIServer {
    pub fn new() -> Self {
        CLIServer { config: ServerConfig::new(), server: ControlServer::new() }
    }

//TODO: Get rid of a tokio usage (not sure, if possible)

    #[tokio::main]
    pub async fn start_server(self, ip: &str, port: &str) {
        let arc_config = std::sync::Arc::new(self.config.get_config());
        let addrs = format!("{}:{}", ip, port);
        let _run_result = russh::server::run(arc_config, addrs, self.server).await;
    }

    /// Authentication methods proposed to the client.
    fn with_auth_metods (mut self, metods: MethodSet) -> Self {
        self.config.set_auth_metods(metods);
        self
    }

    /// The authentication banner, usually a warning message shown to the client.
    fn with_auth_banner (mut self, banner: Option<&'static str>) -> Self  {
        self.config.set_auth_banner(banner);
        self
    }

    /// Authentication rejections must happen in constant time for
    /// security reasons. Russh does not handle this by default.
    fn with_auth_rejection_time (mut self, rejection_time: std::time::Duration) -> Self {
        self.config.set_auth_rejection_time(rejection_time);
        self
    }

    /// Authentication rejection time override for the initial "none" auth attempt.
    /// OpenSSH clients will send an initial "none" auth to probe for authentication methods.
    fn with_auth_rejection_time_initial (mut self, rejection_time_initial: Option<std::time::Duration>) -> Self {
        self.config.set_auth_rejection_time_initial(rejection_time_initial);
        self
    }

    /// The server's keys. The first key pair in the client's preference order will be chosen.
    fn with_keys (mut self, keys: Vec<key::KeyPair>) -> Self {
        self.config.set_keys(keys);
        self
    }

    /// The bytes and time limits before key re-exchange.
    fn with_limits (mut self, limits: Limits) -> Self {
        self.config.set_limits(limits);
        self
    }

    /// The initial size of a channel (used for flow control).
    fn with_window_size (mut self, windos_size: u32) -> Self {
        self.config.set_window_size(windos_size);
        self
    }
    
    /// The maximal size of a single packet.
    fn with_maximum_packet_size (mut self, maximum_packet_size: u32) -> Self {
        self.config.set_maximum_packet_size(maximum_packet_size);
        self
    }

    /// Internal event buffer size
    fn with_event_buffer_size (mut self, event_buffer_size: usize) -> Self {
        self.config.set_event_buffer_size(event_buffer_size);
        self
    }

    /// Lists of preferred algorithms.
    fn with_preferred (mut self, preferred: Preferred) -> Self {
        self.config.set_preferred(preferred);
        self
    }

    /// Maximal number of allowed authentication attempts.
    fn with_max_auth_attempts (mut self, max_auth_attempts: usize) -> Self {
        self.config.set_max_auth_attempts(max_auth_attempts);
        self
    }

    /// Time after which the connection is garbage-collected.
    fn with_connection_timeout (mut self, connection_timeout: Option<std::time::Duration>) -> Self {
        self.config.set_connection_timeout(connection_timeout);
        self
    }
}

impl Default for CLIServer {
    fn default() -> Self {
        CLIServer { config: ServerConfig::default(), server: ControlServer::new() }
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
//TODO: Change Handler type to a reference (Get rid of .clone())
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

    async fn disconnected(self, session: Session) -> Result<(Self, Session), Self::Error> {
        Ok((self, session))
    }

    async fn auth_none(self, user: &str) -> Result<(Self, Auth), Self::Error> {
        Ok((self, Auth::Accept))
    }

    async fn auth_password(self, user: &str, password: &str) -> Result<(Self, Auth), Self::Error> {
        Ok((self, Auth::Accept))
    }

    async fn auth_publickey(self, user: &str, public_key: &key::PublicKey) -> Result<(Self, Auth), Self::Error> {
        Ok((self, Auth::Accept))
    }

    async fn channel_open_session(self, channel: Channel<Msg>, mut session: Session) -> Result<(Self, bool, Session), Self::Error> {
        session.data(channel.id(), CryptoVec::from("Hello from CLI!".to_string()));
        Ok((self, true, session))
    }

    async fn channel_close(self, channel: ChannelId, mut session: Session) -> Result<(Self, Session), Self::Error> {
        session.data(channel, CryptoVec::from("Goodbye, user!".to_string()));
        Ok((self, session))
    }

    async fn data(self, channel: ChannelId, data: &[u8], mut session: Session) -> Result<(Self, Session), Self::Error> {
        let data_cooked = std::str::from_utf8(data).unwrap().to_string();

        //For now just echo everything received
        session.data(channel, CryptoVec::from(data_cooked));

        Ok((self, session))
    }
}