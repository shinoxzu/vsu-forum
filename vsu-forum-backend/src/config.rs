use config::Config;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct AppConfig {
    pub db_connstring: String,
    pub logging: LoggingConfig,
    pub jwt: JwtConfig,
}

#[derive(Deserialize, Debug, Clone)]
pub struct LoggingConfig {
    pub level: String,
    pub filters: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct JwtConfig {
    pub secret_key: String,
}

pub fn load_config(path: &str) -> anyhow::Result<AppConfig> {
    let config = Config::builder()
        .add_source(config::File::with_name(path))
        .add_source(config::Environment::with_prefix("BOT"))
        .build()?;

    Ok(config.try_deserialize::<AppConfig>()?)
}
