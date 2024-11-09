use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Extension, Json,
};
use serde::Deserialize;

use crate::dto::post::{CreatePostDTO, PostDTO};
use crate::{
    dto::{claims::Claims, common::ObjectCreatedDTO},
    errors::ApiError,
    extractors::ValidatedJson,
    models::Post,
    state::ApplicationState,
};

#[derive(Deserialize)]
pub struct GetPostsQueryParams {
    pub topic_id: Option<i64>,
}

pub async fn get_posts(
    Query(query): Query<GetPostsQueryParams>,
    State(state): State<ApplicationState>,
) -> Result<(StatusCode, Json<Vec<PostDTO>>), ApiError> {
    let posts = match query.topic_id {
        Some(topic_id) => {
            sqlx::query_as!(Post, "select * from posts where topic_id = $1", topic_id)
                .fetch_all(&state.db_pool)
                .await
        }
        None => {
            sqlx::query_as!(Post, "select * from posts")
                .fetch_all(&state.db_pool)
                .await
        }
    }
    .map_err(|_| ApiError::InternalServerError)?
    .iter()
    .map(|p| PostDTO {
        id: p.id,
        author_id: p.author_id,
        topic_id: p.topic_id,
        text: p.text.clone(),
    })
    .collect();

    Ok((StatusCode::OK, Json(posts)))
}

pub async fn get_post(
    Path(post_id): Path<i64>,
    State(state): State<ApplicationState>,
) -> Result<(StatusCode, Json<PostDTO>), ApiError> {
    let post = sqlx::query_as!(Post, "select * from posts where id = $1 limit 1", post_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    match post {
        Some(post) => {
            let post_dto = PostDTO {
                id: post.id,
                author_id: post.author_id,
                topic_id: post.topic_id,
                text: post.text,
            };
            Ok((StatusCode::OK, Json(post_dto)))
        }
        None => Err(ApiError::OtherError(
            StatusCode::NOT_FOUND,
            "post not found".to_string(),
        )),
    }
}

pub async fn create_post(
    State(state): State<ApplicationState>,
    Extension(claims): Extension<Claims>,
    ValidatedJson(create_post_dto): ValidatedJson<CreatePostDTO>,
) -> Result<(StatusCode, Json<ObjectCreatedDTO>), ApiError> {
    let result = sqlx::query_scalar!(
        "insert into posts(author_id, topic_id, text) values ($1, $2, $3) returning id",
        claims.user_id,
        create_post_dto.topic_id,
        create_post_dto.text
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| ApiError::InternalServerError)?;

    Result::Ok((StatusCode::CREATED, Json(ObjectCreatedDTO { id: result })))
}
