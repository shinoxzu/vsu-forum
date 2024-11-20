use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};

use crate::{
    dto::{bookmark::BookmarkDTO, claims::Claims},
    errors::ApiError,
    models::Bookmark,
    state::ApplicationState,
};

pub async fn get_bookmarks(
    Extension(claims): Extension<Claims>,
    State(state): State<ApplicationState>,
) -> Result<(StatusCode, Json<Vec<BookmarkDTO>>), ApiError> {
    let bookmarks = sqlx::query_as!(
        Bookmark,
        "select * from bookmarks where user_id = $1",
        claims.user_id
    )
    .fetch_all(&state.db_pool)
    .await
    .map_err(|_| ApiError::InternalServerError)?
    .iter()
    .map(|p| BookmarkDTO {
        topic_id: p.topic_id,
    })
    .collect();

    Ok((StatusCode::OK, Json(bookmarks)))
}

pub async fn create_bookmark(
    Path(topic_id): Path<i64>,
    State(state): State<ApplicationState>,
    Extension(claims): Extension<Claims>,
) -> Result<(StatusCode, Json<BookmarkDTO>), ApiError> {
    sqlx::query!(
        "insert into bookmarks(user_id, topic_id) values ($1, $2)",
        claims.user_id,
        topic_id,
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| ApiError::InternalServerError)?;

    Result::Ok((StatusCode::CREATED, Json(BookmarkDTO { topic_id })))
}

pub async fn remove_bookmark(
    Path(topic_id): Path<i64>,
    State(state): State<ApplicationState>,
    Extension(claims): Extension<Claims>,
) -> Result<StatusCode, ApiError> {
    let rows_affected = sqlx::query!(
        "delete from bookmarks where user_id = $1 and topic_id = $2",
        claims.user_id,
        topic_id
    )
    .execute(&state.db_pool)
    .await
    .map_err(|_| ApiError::InternalServerError)?
    .rows_affected();

    if rows_affected > 0 {
        Result::Ok(StatusCode::OK)
    } else {
        Err(ApiError::NotFound(
            "you did not bookmark this topic".to_string(),
        ))
    }
}
