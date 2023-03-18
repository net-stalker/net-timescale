pub struct ServerConfig {
    russh_config: russh::server::Config
}

impl ServerConfig {
    pub (super) fn new() -> Self {
        ServerConfig { russh_config: russh::server::Config::default() }
    }

    pub (super) fn get_config(self) -> russh::server::Config {
        self.russh_config
    }

    pub (super) fn set_auth_metods (&mut self, metods: russh::MethodSet) {
        self.russh_config.methods = metods;
    }

    pub (super) fn set_auth_banner (&mut self, banner: Option<&'static str>) {
        self.russh_config.auth_banner = banner;
    }

    pub (super) fn set_auth_rejection_time (&mut self, rejection_time: std::time::Duration) {
        self.russh_config.auth_rejection_time = rejection_time;
    }

    pub (super) fn set_auth_rejection_time_initial (&mut self, rejection_time_initial: Option<std::time::Duration>) {
        self.russh_config.auth_rejection_time_initial = rejection_time_initial;
    }

    pub (super) fn set_keys (&mut self, keys: Vec<russh_keys::key::KeyPair>) {
        self.russh_config.keys = keys;
    }

    pub (super) fn set_limits (&mut self, limits: russh::Limits) {
        self.russh_config.limits = limits;
    }

    pub (super) fn set_window_size (&mut self, windos_size: u32) {
        self.russh_config.window_size = windos_size;
    }
    
    pub (super) fn set_maximum_packet_size (&mut self, maximum_packet_size: u32) {
        self.russh_config.maximum_packet_size = maximum_packet_size;
    }

    pub (super) fn set_event_buffer_size (&mut self, event_buffer_size: usize) {
        self.russh_config.event_buffer_size = event_buffer_size;
    }

    pub (super) fn set_preferred (&mut self, preferred: russh::Preferred) {
        self.russh_config.preferred = preferred;
    }

    pub (super) fn set_max_auth_attempts (&mut self, max_auth_attempts: usize) {
        self.russh_config.max_auth_attempts = max_auth_attempts;
    }

    pub (super) fn set_connection_timeout (&mut self, connection_timeout: Option<std::time::Duration>) {
        self.russh_config.connection_timeout = connection_timeout;
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        let mut russh_config = russh::server::Config::default();

        russh_config.methods = russh::MethodSet::NONE; 
        russh_config.connection_timeout = None;
        russh_config.auth_rejection_time = std::time::Duration::from_secs(30);

        let path_to_the_secret_key = concat!(env!("CARGO_MANIFEST_DIR"), "/id_ed25519");
        let russh_key_pair = russh_keys::load_secret_key(path_to_the_secret_key, None).unwrap();
        russh_config.keys.push(russh_key_pair);

        ServerConfig { 
             russh_config
        }
    }
}