use config::{builder::DefaultState, Config, ConfigBuilder, Environment, File};
use std::env;

pub fn load_config() -> Config {
    let builder: ConfigBuilder<DefaultState> = ConfigBuilder::default();

    // Construire la configuration
    let config = builder
        .add_source(File::with_name("config/default.yaml"))
        .add_source(Environment::with_prefix("APP"))
        .build()
        .unwrap();

    config
}
