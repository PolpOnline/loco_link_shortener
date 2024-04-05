use axum::http::StatusCode;
use loco_rs::{controller::ErrorDetail, prelude::*};
use serde::Deserialize;
use tracing::error;

use crate::{
    controllers::utils::get_user_from_jwt,
    models::{_entities::links, links::delete::DeleteError},
};

#[derive(Deserialize)]
pub struct DeleteRequest {
    pub shortened: String,
}

/// Deletes a link
pub async fn delete(
    jwt: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<DeleteRequest>,
) -> Result<impl IntoResponse> {
    let user = get_user_from_jwt(&ctx, jwt).await?;

    links::Model::delete(&ctx.db, &params.shortened, user.id)
        .await
        .map_err(|err| {
            let status_code;
            let err_shorthand;
            let mut err_desc = err.to_string();

            match err {
                DeleteError::NotFound => {
                    status_code = StatusCode::NOT_FOUND;
                    err_shorthand = "NOT_FOUND";
                }
                DeleteError::UserIdNotMatching => {
                    status_code = StatusCode::UNAUTHORIZED;
                    err_shorthand = "SHORTENED_NOT_MATCHING_USER";
                }
                _ => {
                    error!("Error deleting: {:?}", err);
                    status_code = StatusCode::INTERNAL_SERVER_ERROR;
                    err_shorthand = "INTERNAL_SERVER_ERROR";
                    err_desc = "Internal server error".to_string();
                }
            }

            Error::CustomError(status_code, ErrorDetail::new(err_shorthand, &err_desc))
        })?;

    Ok(StatusCode::OK)
}
