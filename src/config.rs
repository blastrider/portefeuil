use config::{builder::DefaultState, Config, ConfigBuilder, Environment, File};

const CONF_PATH: &str = "/app/config/default.yaml";

pub fn load_config() -> Config {
    let builder: ConfigBuilder<DefaultState> = ConfigBuilder::default();

    // Construire la configuration
    let config = builder
        .add_source(File::with_name(CONF_PATH))
        .add_source(Environment::with_prefix("APP"))
        .build()
        .unwrap();

    config
}
