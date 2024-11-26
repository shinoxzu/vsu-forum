use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    dto::{
        available_reaction::{
            AvailableReactionDTO, CreateAvailableReactionDTO, UpdateAvailableReactionDTO,
        },
        common::ObjectCreatedDTO,
    },
    errors::ApiError,
    extractors::ValidatedJson,
    models::AvailableReaction,
    state::ApplicationState,
};

pub async fn get_available_reactions(
    State(state): State<ApplicationState>,
) -> Result<(StatusCode, Json<Vec<AvailableReactionDTO>>), ApiError> {
    let reactions = sqlx::query_as!(AvailableReaction, "SELECT * FROM available_reactions")
        .fetch_all(&state.db_pool)
        .await
        .map_err(|_| ApiError::InternalServerError)?
        .iter()
        .map(|r| AvailableReactionDTO {
            id: r.id,
            reaction: r.reaction.clone(),
        })
        .collect();

    Ok((StatusCode::OK, Json(reactions)))
}

pub async fn create_available_reaction(
    State(state): State<ApplicationState>,
    ValidatedJson(create_dto): ValidatedJson<CreateAvailableReactionDTO>,
) -> Result<(StatusCode, Json<ObjectCreatedDTO>), ApiError> {
    let result = sqlx::query_scalar!(
        "INSERT INTO available_reactions (reaction) VALUES ($1) RETURNING id",
        create_dto.reaction
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| ApiError::InternalServerError)?;

    Ok((StatusCode::CREATED, Json(ObjectCreatedDTO { id: result })))
}

pub async fn patch_available_reaction(
    Path(id): Path<i64>,
    State(state): State<ApplicationState>,
    ValidatedJson(update_dto): ValidatedJson<UpdateAvailableReactionDTO>,
) -> Result<StatusCode, ApiError> {
    let rows_affected = if let Some(reaction) = update_dto.reaction {
        sqlx::query!(
            "update available_reactions set reaction = $1 where id = $2",
            reaction,
            id
        )
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
            "reaction with such id not found".to_string(),
        ))
    }
}

pub async fn delete_available_reaction(
    Path(id): Path<i64>,
    State(state): State<ApplicationState>,
) -> Result<StatusCode, ApiError> {
    sqlx::query!("DELETE FROM available_reactions WHERE id = $1", id)
        .execute(&state.db_pool)
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    Ok(StatusCode::OK)
}
