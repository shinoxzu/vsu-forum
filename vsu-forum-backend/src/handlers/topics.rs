use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};

use crate::{
    dto::{
        claims::Claims,
        common::ObjectCreatedDTO,
        topic::{CreateTopicDTO, TopicDTO},
    },
    errors::ApiError,
    extractors::ValidatedJson,
    state::ApplicationState,
};

pub async fn get_topics(
    State(state): State<ApplicationState>,
) -> Result<(StatusCode, Json<Vec<TopicDTO>>), ApiError> {
    let topics = sqlx::query_as!(Topic, "select * from topics")
        .fetch_all(&state.db_pool)
        .await
        .map_err(|_| ApiError::InternalServerError)?
        .iter()
        .map(|t| TopicDTO {
            id: t.id,
            author_id: t.author_id,
            category_id: t.category_id,
            name: t.name.clone(),
        })
        .collect();

    Ok((StatusCode::OK, Json(topics)))
}

pub async fn get_topic(
    Path(topic_id): Path<i64>,
    State(state): State<ApplicationState>,
) -> Result<(StatusCode, Json<TopicDTO>), ApiError> {
    let topic = sqlx::query_as!(
        Topic,
        "select * from topics where id = $1 limit 1",
        topic_id
    )
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| ApiError::InternalServerError)?;

    match topic {
        Some(topic) => {
            let topic_dto = TopicDTO {
                id: topic.id,
                author_id: topic.author_id,
                category_id: topic.category_id,
                name: topic.name,
            };
            Ok((StatusCode::OK, Json(topic_dto)))
        }
        None => Err(ApiError::OtherError(
            StatusCode::NOT_FOUND,
            "topic not found".to_string(),
        )),
    }
}

pub async fn create_topic(
    State(state): State<ApplicationState>,
    Extension(claims): Extension<Claims>,
    ValidatedJson(create_topic_dto): ValidatedJson<CreateTopicDTO>,
) -> Result<(StatusCode, Json<ObjectCreatedDTO>), ApiError> {
    let result = sqlx::query_scalar!(
        "insert into topics(author_id, category_id, name) values ($1, $2, $3) returning id",
        claims.user_id,
        create_topic_dto.category_id,
        create_topic_dto.name
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| ApiError::InternalServerError)?;

    Result::Ok((StatusCode::CREATED, Json(ObjectCreatedDTO { id: result })))
}
