use axum::http::StatusCode;
use loco_rs::{controller::ErrorDetail, prelude::*};

use serde::{Deserialize, Serialize};
use tracing::error;

use crate::models::_entities::links;
use crate::models::links::info::InfoError;

#[derive(Deserialize)]
pub struct InfoRequest {
    pub shortened: String,
}

#[derive(Serialize)]
pub struct InfoResponse {
    pub original: String,
    pub clicks: i32,
    pub created_at: String,
}

pub async fn info(
    State(ctx): State<AppContext>,
    Json(params): Json<InfoRequest>,
) -> Result<impl IntoResponse> {
    let link = links::Model::info(&ctx.db, &params.shortened)
        .await
        .map_err(|err| {
            let status_code;
            let err_shorthand;

            match err {
                InfoError::NotFound => {
                    status_code = StatusCode::NOT_FOUND;
                    err_shorthand = "NOT_FOUND";
                }
                _ => {
                    error!("Error getting info: {:?}", err);
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

    Ok(Json(InfoResponse::from(link)))
}

impl From<links::Model> for InfoResponse {
    fn from(model: links::Model) -> Self {
        InfoResponse {
            original: model.original,
            clicks: model.clicks,
            created_at: model.created_at.to_string(),
        }
    }
}