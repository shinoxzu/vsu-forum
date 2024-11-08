use config::Config;
use serde::Deserialize;
use sha3::{Digest, Sha3_256};

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

pub fn hash_text(text: String) -> Vec<u8> {
    let mut hasher = Sha3_256::new();
    hasher.update(text.as_bytes());
    hasher.finalize().to_vec()
}

pub fn load_config(path: &str) -> anyhow::Result<AppConfig> {
    let config = Config::builder()
        .add_source(config::File::with_name(path))
        .add_source(config::Environment::with_prefix("BOT"))
        .build()?;

    Ok(config.try_deserialize::<AppConfig>()?)
}
