use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};

use crate::{
    dto::{
        claims::Claims,
        common::ObjectCreatedDTO,
        topic::{CreateTopicDTO, TopicDTO, UpdateTopicDTO},
        topic_category::TopicCategoryDTO,
        user::UserDTO,
    },
    errors::ApiError,
    extractors::ValidatedJson,
    state::ApplicationState,
};

pub async fn get_topics(
    State(state): State<ApplicationState>,
) -> Result<(StatusCode, Json<Vec<TopicDTO>>), ApiError> {
    let topics = sqlx::query!(
        "
        SELECT
            t.id AS topic_id,
            t.category_id AS category_id,
            t.name AS topic_name,
            u.id AS creator_id,
            u.login AS creator_login,
            tc.name AS category_name,
            COUNT(p.id) AS posts_count
        FROM
            topics t
        JOIN
            users u ON t.author_id = u.id
        JOIN
            topics_categories tc ON t.category_id = tc.id
        LEFT JOIN
            posts p ON t.id = p.topic_id
        GROUP BY
            t.id, t.author_id, t.category_id, t.name, u.id, u.login, tc.name;
        "
    )
    .fetch_all(&state.db_pool)
    .await
    .map_err(|_| ApiError::InternalServerError)?
    .iter()
    .map(|record| TopicDTO {
        id: record.topic_id,
        name: record.topic_name.clone(),
        category: TopicCategoryDTO {
            id: record.category_id,
            name: record.category_name.clone(),
        },
        creator: UserDTO {
            id: record.creator_id,
            login: record.creator_login.clone(),
        },
        posts_count: record.posts_count.unwrap_or(0),
    })
    .collect();

    Ok((StatusCode::OK, Json(topics)))
}

pub async fn get_topic(
    Path(id): Path<i64>,
    State(state): State<ApplicationState>,
) -> Result<(StatusCode, Json<TopicDTO>), ApiError> {
    let record = sqlx::query!(
        "
        SELECT
            t.id AS topic_id,
            t.category_id AS category_id,
            t.name AS topic_name,
            u.id AS creator_id,
            u.login AS creator_login,
            tc.name AS category_name,
            COUNT(p.id) AS posts_count
        FROM
            topics t
        JOIN
            users u ON t.author_id = u.id
        JOIN
            topics_categories tc ON t.category_id = tc.id
        LEFT JOIN
            posts p ON t.id = p.topic_id
        WHERE
            t.id = $1
        GROUP BY
            t.id, t.author_id, t.category_id, t.name, u.id, u.login, tc.name;
        ",
        id
    )
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| ApiError::InternalServerError)?;

    match record {
        Some(record) => {
            let topic_dto = TopicDTO {
                id: record.topic_id,
                name: record.topic_name.clone(),
                category: TopicCategoryDTO {
                    id: record.category_id,
                    name: record.category_name.clone(),
                },
                creator: UserDTO {
                    id: record.creator_id,
                    login: record.creator_login.clone(),
                },
                posts_count: record.posts_count.unwrap_or(0),
            };
            Ok((StatusCode::OK, Json(topic_dto)))
        }
        None => Err(ApiError::NotFound("topic not found".to_string())),
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

pub async fn remove_topic(
    Path(id): Path<i64>,
    State(state): State<ApplicationState>,
) -> Result<StatusCode, ApiError> {
    let rows_affected = sqlx::query!("delete from topics where id = $1", id)
        .execute(&state.db_pool)
        .await
        .map_err(|_| ApiError::InternalServerError)?
        .rows_affected();

    if rows_affected > 0 {
        Result::Ok(StatusCode::OK)
    } else {
        Err(ApiError::NotFound(
            "topic with such id not found".to_string(),
        ))
    }
}

pub async fn patch_topic(
    Path(topic_id): Path<i64>,
    State(state): State<ApplicationState>,
    ValidatedJson(update_topic_dto): ValidatedJson<UpdateTopicDTO>,
) -> Result<StatusCode, ApiError> {
    let mut rows_affected = 0;

    if let Some(name) = update_topic_dto.name {
        rows_affected = sqlx::query!("update topics set name = $1 where id = $2", name, topic_id)
            .execute(&state.db_pool)
            .await
            .map_err(|_| ApiError::InternalServerError)?
            .rows_affected();
    }
    if let Some(category_id) = update_topic_dto.category_id {
        rows_affected = sqlx::query!(
            "update topics set category_id = $1 where id = $2",
            category_id,
            topic_id
        )
        .execute(&state.db_pool)
        .await
        .map_err(|_| ApiError::InternalServerError)?
        .rows_affected();
    }

    if rows_affected > 0 {
        Result::Ok(StatusCode::OK)
    } else {
        Err(ApiError::NotFound(
            "topic with such id not found".to_string(),
        ))
    }
}
