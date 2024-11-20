use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Extension, Json,
};

use crate::{
    dto::{
        claims::Claims,
        common::ObjectCreatedDTO,
        post::{CreatePostDTO, GetPostsDTO, PostDTO, UpdatePostDTO},
    },
    errors::ApiError,
    extractors::ValidatedJson,
    models::Post,
    state::ApplicationState,
};

pub async fn get_posts(
    Query(query): Query<GetPostsDTO>,
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
    Path(id): Path<i64>,
    State(state): State<ApplicationState>,
) -> Result<(StatusCode, Json<PostDTO>), ApiError> {
    let post = sqlx::query_as!(Post, "select * from posts where id = $1 limit 1", id)
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
        None => Err(ApiError::NotFound("post not found".to_string())),
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

pub async fn remove_post(
    Path(id): Path<i64>,
    State(state): State<ApplicationState>,
) -> Result<StatusCode, ApiError> {
    let rows_affected = sqlx::query!("delete from posts where id = $1", id)
        .execute(&state.db_pool)
        .await
        .map_err(|_| ApiError::InternalServerError)?
        .rows_affected();

    if rows_affected > 0 {
        Result::Ok(StatusCode::OK)
    } else {
        Err(ApiError::NotFound(
            "post with such id not found".to_string(),
        ))
    }
}

pub async fn patch_post(
    Path(post_id): Path<i64>,
    State(state): State<ApplicationState>,
    ValidatedJson(update_post_dto): ValidatedJson<UpdatePostDTO>,
) -> Result<StatusCode, ApiError> {
    let rows_affected = if let Some(text) = update_post_dto.text {
        sqlx::query!("update posts set text = $1 where id = $2", text, post_id)
            .execute(&state.db_pool)
            .await
            .map_err(|_| ApiError::InternalServerError)?
            .rows_affected()
    } else {
        0
    };

    if rows_affected > 0 {
        Result::Ok(StatusCode::OK)
    } else {
        Err(ApiError::NotFound(
            "post with such id not found".to_string(),
        ))
    }
}
