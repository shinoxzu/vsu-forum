use axum::extract::Path;
use axum::Json;
use axum::{extract::State, http::StatusCode, Extension};

use crate::dto::reactions::ReactionDTO;
use crate::models::Reaction;
use crate::{dto::claims::Claims, errors::ApiError, state::ApplicationState};

pub async fn get_reactions(
    Path(post_id): Path<i64>,
    State(state): State<ApplicationState>,
) -> Result<(StatusCode, Json<Vec<ReactionDTO>>), ApiError> {
    let reactions = sqlx::query_as!(
        Reaction,
        "select * from reactions where post_id = $1",
        post_id,
    )
    .fetch_all(&state.db_pool)
    .await
    .map_err(|_| ApiError::InternalServerError)?
    .iter()
    .map(|r| ReactionDTO {
        author_id: r.author_id,
        reaction: r.reaction.clone(),
    })
    .collect();

    Ok((StatusCode::OK, Json(reactions)))
}

pub async fn add_reaction(
    Path(post_id): Path<i64>,
    Path(reaction): Path<String>,
    State(state): State<ApplicationState>,
    Extension(claims): Extension<Claims>,
) -> Result<StatusCode, ApiError> {
    sqlx::query!(
        "insert into reactions(post_id, author_id, reaction) values ($1, $2, $3)",
        post_id,
        claims.user_id,
        reaction,
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| ApiError::InternalServerError)?;

    Result::Ok(StatusCode::CREATED)
}

pub async fn remove_reaction(
    Path(post_id): Path<i64>,
    Path(reaction): Path<String>,
    State(state): State<ApplicationState>,
    Extension(claims): Extension<Claims>,
) -> Result<StatusCode, ApiError> {
    let rows_affected = sqlx::query!(
        "delete from reactions where post_id = $1 and author_id = $2 and reaction = $3",
        post_id,
        claims.user_id,
        reaction,
    )
    .execute(&state.db_pool)
    .await
    .map_err(|_| ApiError::InternalServerError)?
    .rows_affected();

    if rows_affected > 0 {
        Result::Ok(StatusCode::OK)
    } else {
        Err(ApiError::NotFound(
            "you did not set this reaction".to_string(),
        ))
    }
}
