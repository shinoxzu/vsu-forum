use sqlx::PgPool;

use crate::tools::AppConfig;

#[derive(Debug, Clone)]
pub struct ApplicationState {
    pub config: AppConfig,
    pub db_pool: PgPool,
}
