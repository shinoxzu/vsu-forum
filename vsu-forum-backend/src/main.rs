pub mod handlers;
pub mod models;
pub mod dto;
pub mod tools;
pub mod state;

use std::{path::Path, str::FromStr};

use env_logger::{Builder, Target};
use log::LevelFilter;
use sqlx::{
    postgres::PgPoolOptions,
    migrate::Migrator
};
use anyhow::Context;
use axum::{routing::get,Router};

use handlers::users::{get_user, login_user, register_user};
use state::ApplicationState;



#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config_path = std::env::var("CONFIG_PATH")
        .context("cannot load config path from env variable")?;
    let config = tools::load_config(&config_path)
        .context("cannot load config")?;
    
    Builder::new()
        .filter_level(LevelFilter::from_str(&config.log_level).context("cannot parse log level")?)
        .parse_filters(&config.log_filters)
        .parse_default_env()
        .target(Target::Stdout)
        .init();
    
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&config.db_connstring)
        .await
        .context("cannot connect to the database")?;
    
    Migrator::new(Path::new("./migrations"))
        .await
        .expect("cannot load migtations")
        .run(&pool)
        .await
        .expect("cannot run migrations");
    
    let app = Router::new()
        .route("/users/register", get(register_user))
        .route("/users/login", get(login_user))
        .route("/users/:id", get(get_user))
        .layer(tower_http::catch_panic::CatchPanicLayer::new())
        .with_state(ApplicationState { config });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .context("failed to bind TCP listener")?;
    axum::serve(listener, app)
        .await
        .context("axum::serve failed")?;
    
    Ok(())
}
