use axum::{extract::State, http::StatusCode, Extension, Json};

use crate::dto::bookmark::{BookmarkDTO, CreateBookmarkDTO};
use crate::models::Bookmark;
use crate::{
    dto::claims::Claims, errors::ApiError, extractors::ValidatedJson, state::ApplicationState,
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
    State(state): State<ApplicationState>,
    Extension(claims): Extension<Claims>,
    ValidatedJson(create_bookmark_dto): ValidatedJson<CreateBookmarkDTO>,
) -> Result<(StatusCode, Json<BookmarkDTO>), ApiError> {
    sqlx::query!(
        "insert into bookmarks(user_id, topic_id) values ($1, $2)",
        claims.user_id,
        create_bookmark_dto.topic_id,
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| ApiError::InternalServerError)?;

    Result::Ok((
        StatusCode::CREATED,
        Json(BookmarkDTO {
            topic_id: create_bookmark_dto.topic_id,
        }),
    ))
}
