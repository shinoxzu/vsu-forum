use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    dto::{
        common::ObjectCreatedDTO,
        topic_category::{CreateTopicCategoryDTO, TopicCategoryDTO},
    },
    errors::ApiError,
    extractors::ValidatedJson,
    models::TopicCategory,
    state::ApplicationState,
};

pub async fn get_topic_categories(
    State(state): State<ApplicationState>,
) -> Result<(StatusCode, Json<Vec<TopicCategoryDTO>>), ApiError> {
    let topics_categories = sqlx::query_as!(TopicCategory, "select * from topics_categories")
        .fetch_all(&state.db_pool)
        .await
        .map_err(|_| ApiError::InternalServerError)?
        .iter()
        .map(|t| TopicCategoryDTO {
            id: t.id,
            name: t.name.clone(),
        })
        .collect();

    Ok((StatusCode::OK, Json(topics_categories)))
}

pub async fn get_topic_category(
    Path(topic_category_id): Path<i64>,
    State(state): State<ApplicationState>,
) -> Result<(StatusCode, Json<TopicCategoryDTO>), ApiError> {
    let topic_category = sqlx::query_as!(
        TopicCategory,
        "select * from topics_categories where id = $1 limit 1",
        topic_category_id
    )
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| ApiError::InternalServerError)?;

    match topic_category {
        Some(topic) => {
            let topic_category_dto = TopicCategoryDTO {
                id: topic.id,
                name: topic.name,
            };
            Ok((StatusCode::OK, Json(topic_category_dto)))
        }
        None => Err(ApiError::NotFound("topic not found".to_string())),
    }
}

pub async fn create_topic_category(
    State(state): State<ApplicationState>,
    ValidatedJson(create_category_dto): ValidatedJson<CreateTopicCategoryDTO>,
) -> Result<(StatusCode, Json<ObjectCreatedDTO>), ApiError> {
    let result = sqlx::query_scalar!(
        "insert into topics_categories(name) values ($1) returning id",
        create_category_dto.name
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| ApiError::InternalServerError)?;

    Result::Ok((StatusCode::CREATED, Json(ObjectCreatedDTO { id: result })))
}
