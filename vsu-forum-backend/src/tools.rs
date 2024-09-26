use config::Config;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct AppConfig {
    pub db_connstring: String,
    pub log_level: String,
    pub log_filters: String,
}

pub fn load_config(path: &str) -> anyhow::Result<AppConfig> {
    let config = Config::builder()
        .add_source(config::File::with_name(path))
        .add_source(config::Environment::with_prefix("BOT"))
        .build()?;

    Ok(config.try_deserialize::<AppConfig>()?)
}
