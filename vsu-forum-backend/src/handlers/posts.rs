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
        user::UserDTO,
    },
    errors::ApiError,
    extractors::ValidatedJson,
    state::ApplicationState,
};

pub async fn get_posts(
    Query(query): Query<GetPostsDTO>,
    State(state): State<ApplicationState>,
) -> Result<(StatusCode, Json<Vec<PostDTO>>), ApiError> {
    let posts = sqlx::query!(
        "
        SELECT
            p.id AS post_id,
            p.topic_id AS topic_id,
            p.created_at as created_at,
            p.text AS post_text,
            u.id AS sender_id,
            u.login AS sender_login
        FROM
            posts p
        JOIN
            users u ON p.author_id = u.id
        WHERE
            p.topic_id = $1;
        ",
        query.topic_id
    )
    .fetch_all(&state.db_pool)
    .await
    .map_err(|_| ApiError::InternalServerError)?
    .iter()
    .map(|record| PostDTO {
        id: record.post_id,
        created_at: record.created_at,
        sender: UserDTO {
            id: record.sender_id,
            login: record.sender_login.clone(),
        },
        topic_id: record.topic_id,
        text: record.post_text.clone(),
    })
    .collect();

    Ok((StatusCode::OK, Json(posts)))
}

pub async fn get_post(
    Path(id): Path<i64>,
    State(state): State<ApplicationState>,
) -> Result<(StatusCode, Json<PostDTO>), ApiError> {
    let record = sqlx::query!(
        "
        SELECT
        p.id AS post_id,
        p.topic_id AS topic_id,
        p.created_at as created_at,
        p.text AS post_text,
        u.id AS sender_id,
        u.login AS sender_login
        FROM
            posts p
        JOIN
            users u ON p.author_id = u.id
        WHERE
            p.id = $1;
        ",
        id
    )
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| ApiError::InternalServerError)?;

    match record {
        Some(record) => {
            let post_dto = PostDTO {
                id: record.post_id,
                created_at: record.created_at,
                sender: UserDTO {
                    id: record.sender_id,
                    login: record.sender_login.clone(),
                },
                topic_id: record.topic_id,
                text: record.post_text.clone(),
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
