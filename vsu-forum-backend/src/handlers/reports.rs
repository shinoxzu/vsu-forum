use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};

use crate::{
    dto::{
        claims::Claims,
        common::ObjectCreatedDTO,
        report::{CreateReportDTO, ReportDTO, UpdateReportDTO},
    },
    errors::ApiError,
    extractors::ValidatedJson,
    models::Report,
    state::ApplicationState,
};

pub async fn get_reports(
    State(state): State<ApplicationState>,
) -> Result<(StatusCode, Json<Vec<ReportDTO>>), ApiError> {
    let reports = sqlx::query_as!(Report, "select * from reports")
        .fetch_all(&state.db_pool)
        .await
        .map_err(|_| ApiError::InternalServerError)?
        .iter()
        .map(|r| ReportDTO {
            id: r.id,
            author_id: r.author_id,
            reported_user_id: r.reported_user_id,
            reason: r.reason.clone(),
        })
        .collect();

    Ok((StatusCode::OK, Json(reports)))
}

pub async fn get_report(
    Path(id): Path<i64>,
    State(state): State<ApplicationState>,
) -> Result<(StatusCode, Json<ReportDTO>), ApiError> {
    let report = sqlx::query_as!(Report, "select * from reports where id = $1 limit 1", id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    match report {
        Some(report) => {
            let report_dto = ReportDTO {
                id: report.id,
                author_id: report.author_id,
                reported_user_id: report.reported_user_id,
                reason: report.reason,
            };
            Ok((StatusCode::OK, Json(report_dto)))
        }
        None => Err(ApiError::NotFound("report not found".to_string())),
    }
}

pub async fn create_report(
    State(state): State<ApplicationState>,
    Extension(claims): Extension<Claims>,
    ValidatedJson(create_report_dto): ValidatedJson<CreateReportDTO>,
) -> Result<(StatusCode, Json<ObjectCreatedDTO>), ApiError> {
    let result = sqlx::query_scalar!(
        "insert into reports(author_id, reported_user_id, reason) values ($1, $2, $3) returning id",
        claims.user_id,
        create_report_dto.reported_user_id,
        create_report_dto.reason
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| ApiError::InternalServerError)?;

    Result::Ok((StatusCode::CREATED, Json(ObjectCreatedDTO { id: result })))
}

pub async fn remove_report(
    Path(id): Path<i64>,
    State(state): State<ApplicationState>,
) -> Result<StatusCode, ApiError> {
    let rows_affected = sqlx::query!("delete from reports where id = $1", id)
        .execute(&state.db_pool)
        .await
        .map_err(|_| ApiError::InternalServerError)?
        .rows_affected();

    if rows_affected > 0 {
        Result::Ok(StatusCode::OK)
    } else {
        Err(ApiError::NotFound(
            "report with such id not found".to_string(),
        ))
    }
}

pub async fn patch_report(
    Path(report_id): Path<i64>,
    State(state): State<ApplicationState>,
    ValidatedJson(update_report_dto): ValidatedJson<UpdateReportDTO>,
) -> Result<StatusCode, ApiError> {
    let rows_affected = if let Some(reason) = update_report_dto.reason {
        sqlx::query!(
            "update reports set reason = $1 where id = $2",
            reason,
            report_id
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
            "report with such id not found".to_string(),
        ))
    }
}
