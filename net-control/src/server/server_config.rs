pub struct ServerConfig {
    russh_config: russh::server::Config,

    server_ip: &'static str,
    server_port: &'static str
}

impl ServerConfig {
    pub (super) fn get_config(self) -> russh::server::Config {
        self.russh_config
    }
    pub (super) fn get_server_ip(& self) -> &'static str {
        self.server_ip
    }
    pub (super) fn get_server_port(& self) -> &'static str {
        self.server_port
    }

    pub fn builder() -> ServerConfigBuilder {
        ServerConfigBuilder::new()
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        let path_to_the_secret_key = concat!(env!("CARGO_MANIFEST_DIR"), "/id_ed25519");
        let russh_key_pair = russh_keys::load_secret_key(path_to_the_secret_key, None).unwrap();

        ServerConfig::builder()
            .with_ip("0.0.0.0")
            .with_port("2222")
            .with_auth_metods(russh::MethodSet::NONE)
            .with_connection_timeout(None)
            .with_auth_rejection_time(std::time::Duration::from_secs(30))
            .with_keys(vec!(russh_key_pair))
            .build()
    }
}

pub struct ServerConfigBuilder {
    russh_config: Option<russh::server::Config>,

    server_ip: Option<&'static str>,
    server_port: Option<&'static str>
}

impl ServerConfigBuilder {
    pub fn new() -> Self {
        ServerConfigBuilder { 
            russh_config: Some(russh::server::Config::default()),

            server_ip: Some("0.0.0.0"),
            server_port: Some("2222") 
        }
    }

    pub fn with_ip(mut self, ip: &'static str) -> Self {
        self.server_ip = Some(ip);
        self
    }
    pub fn with_port(mut self, port: &'static str) -> Self {
        self.server_port = Some(port);
        self
    }

    pub fn with_auth_metods (mut self, metods: russh::MethodSet) -> Self {
        self.russh_config.as_mut().unwrap().methods = metods;
        self
    }
    pub fn with_auth_banner (mut self, banner: Option<&'static str>) -> Self {
        self.russh_config.as_mut().unwrap().auth_banner = banner;
        self
    }
    pub fn with_auth_rejection_time (mut self, rejection_time: std::time::Duration) -> Self {
        self.russh_config.as_mut().unwrap().auth_rejection_time = rejection_time;
        self
    }
    pub fn with_auth_rejection_time_initial (mut self, rejection_time_initial: Option<std::time::Duration>) -> Self {
        self.russh_config.as_mut().unwrap().auth_rejection_time_initial = rejection_time_initial;
        self
    }
    pub fn with_keys (mut self, keys: Vec<russh_keys::key::KeyPair>) -> Self {
        self.russh_config.as_mut().unwrap().keys = keys;
        self
    }
    pub fn with_limits (mut self, limits: russh::Limits) -> Self {
        self.russh_config.as_mut().unwrap().limits = limits;
        self
    }
    pub fn with_window_size (mut self, windos_size: u32) -> Self {
        self.russh_config.as_mut().unwrap().window_size = windos_size;
        self
    }
    pub fn with_maximum_packet_size (mut self, maximum_packet_size: u32) -> Self {
        self.russh_config.as_mut().unwrap().maximum_packet_size = maximum_packet_size;
        self
    }
    pub fn with_event_buffer_size (mut self, event_buffer_size: usize) -> Self {
        self.russh_config.as_mut().unwrap().event_buffer_size = event_buffer_size;
        self
    }
    pub fn with_preferred (mut self, preferred: russh::Preferred) -> Self {
        self.russh_config.as_mut().unwrap().preferred = preferred;
        self
    }
    pub fn with_max_auth_attempts (mut self, max_auth_attempts: usize) -> Self {
        self.russh_config.as_mut().unwrap().max_auth_attempts = max_auth_attempts;
        self
    }
    pub fn with_connection_timeout (mut self, connection_timeout: Option<std::time::Duration>) -> Self {
        self.russh_config.as_mut().unwrap().connection_timeout = connection_timeout;
        self
    }

    pub fn build(self) -> ServerConfig {
        ServerConfig {
            russh_config: self.russh_config.unwrap(),

            server_ip: self.server_ip.unwrap(),
            server_port: self.server_port.unwrap()  
        }
    }
}