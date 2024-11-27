use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};

use crate::{
    dto::{
        search::SearchQueryParamsDTO, topic::TopicDTO, topic_category::TopicCategoryDTO,
        user::UserDTO,
    },
    errors::ApiError,
    state::ApplicationState,
};

pub async fn search(
    State(state): State<ApplicationState>,
    Query(query): Query<SearchQueryParamsDTO>,
) -> Result<(StatusCode, Json<Vec<TopicDTO>>), ApiError> {
    let search_pattern = format!("%{}%", query.query.to_lowercase());

    let topics: Vec<TopicDTO> = sqlx::query!(
        "
        SELECT DISTINCT
            t.id AS topic_id,
            t.category_id AS category_id,
            t.name AS topic_name,
            t.created_at as created_at,
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
            LOWER(t.name) LIKE $1
            OR LOWER(tc.name) LIKE $1
            OR LOWER(u.login) LIKE $1
            OR EXISTS (
                SELECT 1
                FROM posts p2
                WHERE p2.topic_id = t.id
                AND LOWER(p2.text) LIKE $1
            )
        GROUP BY
            t.id, t.author_id, t.category_id, t.name, u.id, u.login, tc.name
        ORDER BY
            t.id DESC
        LIMIT 50
        ",
        search_pattern
    )
    .fetch_all(&state.db_pool)
    .await
    .map_err(|_| ApiError::InternalServerError)?
    .iter()
    .map(|record| TopicDTO {
        id: record.topic_id,
        created_at: record.created_at,
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
