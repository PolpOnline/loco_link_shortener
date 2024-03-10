use axum::http::StatusCode;
use loco_rs::{controller::ErrorDetail, prelude::*};
use serde::Deserialize;
use tracing::error;

use crate::models::{_entities::links, links::delete::DeleteError};

#[derive(Deserialize)]
pub struct DeleteRequest {
    pub shortened: String,
}

pub async fn delete(
    State(ctx): State<AppContext>,
    Json(params): Json<DeleteRequest>,
) -> Result<impl IntoResponse> {
    links::Model::delete(&ctx.db, &params.shortened)
        .await
        .map_err(|err| {
            let status_code;
            let err_shorthand;

            match err {
                DeleteError::NotFound => {
                    status_code = StatusCode::NOT_FOUND;
                    err_shorthand = "NOT_FOUND";
                }
                _ => {
                    error!("Error deleting: {:?}", err);
                    status_code = StatusCode::INTERNAL_SERVER_ERROR;
                    err_shorthand = "INTERNAL_SERVER_ERROR";
                }
            }

            Error::CustomError(
                status_code,
                ErrorDetail {
                    error: Some(err_shorthand.to_string()),
                    description: Some(err.to_string()),
                },
            )
        })?;

    Ok(StatusCode::OK)
}
