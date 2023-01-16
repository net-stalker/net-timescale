use std::sync::Arc;

use zmq::Context;

use net_commons::config::{Config, ConfigManager, ConfigSpec, ConfigFile, FileReader};

pub struct HubContext {
    pub config: Arc<Config>,
    pub zmq_ctx: Arc<Context>,
}

impl HubContext {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Default for HubContext {
    fn default() -> Self {
        let config = Arc::new(ConfigManager { application_name: "net-hub", file_loader: Box::new(ConfigFile) as Box<dyn FileReader> }.load());

        Self {
            config,
            zmq_ctx: Arc::new(Default::default()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_load_default_configuration() {
        let hub_context = HubContext::new();
        let config = hub_context.config;
        println!("{}", config);

        assert_eq!(config.dealer.enable, true);
        assert_eq!(config.dealer.endpoint, "tcp://0.0.0.0:5555");
        assert_eq!(config.data.devices, vec!["any"]);
        assert_eq!(config.data.number_packages, -1);
        assert_eq!(config.data.buffer_size, 1000);

        let context = hub_context.zmq_ctx.clone();
    }
}