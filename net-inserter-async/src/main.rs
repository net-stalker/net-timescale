use net_inserter_async::config::Config;
use net_inserter_async::component::inserter::Inserter;

#[tokio::main]
async fn main() {
    init_log();
    log::info!("Run module");
    
    let config = Config::builder().build().expect("read config error");

    let inserter_component = Inserter::new(config).await;
    
    log::info!("Created component");
    
    inserter_component.run().await;
}

fn init_log() {
    let config_str = include_str!("log4rs.yml");
    let config = serde_yaml::from_str(config_str).unwrap();
    log4rs::init_raw_config(config).unwrap();
}


/*
   Basically what do I need to have from migrator?
   I need it to run migrations at the start of any service
   What do I need to run migrations?
   I need it to have a connection to the database
   I need it to have a list of migrations
   Do I need to create a new crate especially for this? - probably yes
   Do I need to add it to net-core? - no, Don't think so because I don't really need to push it to crates.io
   Of course it needs to run such migrations before starting the service net-isnerter-async ot net-reporter
*/