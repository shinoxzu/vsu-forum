use axum::{extract::State, http::StatusCode, Json};

use crate::{dto::stats::ForumStatsDTO, errors::ApiError, state::ApplicationState};

pub async fn get_stats(
    State(state): State<ApplicationState>,
) -> Result<(StatusCode, Json<ForumStatsDTO>), ApiError> {
    let stats = sqlx::query!(
        r#"
            SELECT
                (SELECT COUNT(*) FROM posts) as posts_count,
                (SELECT COUNT(*) FROM users) as users_count,
                (SELECT COUNT(*) FROM topics) as topics_count
            "#
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| ApiError::InternalServerError)?;

    Ok((
        StatusCode::OK,
        Json(ForumStatsDTO {
            posts_count: stats.posts_count.unwrap_or(0),
            users_count: stats.users_count.unwrap_or(0),
            topics_count: stats.topics_count.unwrap_or(0),
        }),
    ))
}
