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
    routing::{delete, get, post},
    Router,
};
use env_logger::{Builder, Target};
use log::LevelFilter;
use sqlx::{migrate::Migrator, postgres::PgPoolOptions};
use tower_http::cors::CorsLayer;

use handlers::{
    bookmarks::{create_bookmark, get_bookmarks, remove_bookmark},
    posts::{create_post, get_post, get_posts, remove_post},
    reactions::{add_reaction, get_reactions, remove_reaction},
    reports::{create_report, get_report, get_reports, remove_report},
    topics::{create_topic, get_topic, get_topics, remove_topic},
    topics_categories::{
        create_topic_category, get_topic_categories, get_topic_category, remove_topic_category,
    },
    users::{get_me, get_user, login_user, register_user},
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
        .route("/topics-categories/:id", get(get_topic_category))
        .route("/posts", get(get_posts))
        .route("/posts/:id", get(get_post))
        .route("/posts/:post_id/reactions", get(get_reactions))
        .route("/reports", get(get_reports))
        .route("/reports/:id", get(get_report));

    let secure_router = Router::new()
        .route("/users/me", get(get_me))
        .route("/topics", post(create_topic))
        .route("/topics/:topic_id", delete(remove_topic))
        .route("/bookmarks", get(get_bookmarks))
        .route("/topics/:topic_id/bookmark", post(create_bookmark))
        .route("/topics/:topic_id/bookmark", delete(remove_bookmark))
        .route("/topics-categories", post(create_topic_category))
        .route(
            "/topics-categories/:topic_category_id",
            delete(remove_topic_category),
        )
        .route("/posts", post(create_post))
        .route("/posts/:post_id", delete(remove_post))
        .route("/reports", post(create_report))
        .route("/reports/:report_id", delete(remove_report))
        .route("/posts/:post_id/reactions/:reaction", post(add_reaction))
        .route(
            "/posts/:post_id/reactions/:reaction",
            delete(remove_reaction),
        )
        .route_layer(axum::middleware::from_fn_with_state(
            state.clone(),
            middlewares::auth::auth_middleware,
        ));

    let app = Router::new()
        .merge(router)
        .merge(secure_router)
        .with_state(state.clone())
        .layer(CorsLayer::permissive()); // TODO: adjust cors settings

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .context("failed to bind TCP listener")?;

    axum::serve(listener, app)
        .await
        .context("axum::serve failed")?;

    Ok(())
}
