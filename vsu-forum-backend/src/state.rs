use sqlx::PgPool;

use crate::config::AppConfig;

#[derive(Debug, Clone)]
pub struct ApplicationState {
    pub config: AppConfig,
    pub db_pool: PgPool,
}
