pub mod dto;
mod errors;
mod extractors;
pub mod handlers;
mod middlewares;
pub mod models;
pub mod state;
pub mod tools;

use std::{env::var, path::Path, str::FromStr};

use anyhow::Context;
use axum::{
    routing::{get, post},
    Router,
};
use env_logger::{Builder, Target};
use log::LevelFilter;
use sqlx::{migrate::Migrator, postgres::PgPoolOptions};

use handlers::{
    topics::{create_topic, get_topic, get_topics},
    topics_categories::{create_topic_category, get_topic_categories, get_topic_category},
    users::{get_user, login_user, register_user},
};
use state::ApplicationState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config_path = var("CONFIG_PATH").context("cannot load config path from env variable")?;
    let config = tools::load_config(&config_path).context("cannot load config")?;

    Builder::new()
        .filter_level(
            LevelFilter::from_str(&config.logging.level).context("cannot parse log level")?,
        )
        .parse_filters(&config.logging.filters)
        .parse_default_env()
        .target(Target::Stdout)
        .init();

    let db_pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&config.db_connstring)
        .await
        .context("cannot connect to the database")?;

    Migrator::new(Path::new("./migrations"))
        .await
        .expect("cannot load migtations")
        .run(&db_pool)
        .await
        .expect("cannot run migrations");

    let state = ApplicationState {
        config: config.clone(),
        db_pool: db_pool.clone(),
    };

    let router = Router::new()
        .route("/users/register", post(register_user))
        .route("/users/login", post(login_user))
        .route("/users/:id", get(get_user))
        .route("/topics", get(get_topics))
        .route("/topics/:id", get(get_topic))
        .route("/topics-categories", get(get_topic_categories))
        .route("/topics-categories/:id", get(get_topic_category));

    let secure_router = Router::new()
        .route("/topics", post(create_topic))
        .route("/topics-categories", post(create_topic_category))
        .route_layer(axum::middleware::from_fn_with_state(
            state.clone(),
            middlewares::auth::auth_middleware,
        ));

    let app = Router::new()
        .merge(router)
        .merge(secure_router)
        .with_state(state.clone());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .context("failed to bind TCP listener")?;

    axum::serve(listener, app)
        .await
        .context("axum::serve failed")?;

    Ok(())
}
