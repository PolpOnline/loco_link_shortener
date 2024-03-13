use axum::{http::StatusCode, response::Redirect};
use loco_rs::{controller::ErrorDetail, prelude::*};
use tracing::error;

use crate::models::{_entities::links, links::retrieve::RetrieveError};

/// Retrieves the original URL from the shortened URL and redirects to it
pub async fn retrieve(
    State(ctx): State<AppContext>,
    Path(shortened): Path<String>,
) -> Result<impl IntoResponse> {
    let original =
        links::Model::retrieve_original_and_increase_clicks_by_shortened(&ctx.db, shortened)
            .await
            .map_err(|err| {
                let status_code;
                let err_shorthand;

                if let RetrieveError::NotFound = err {
                    status_code = StatusCode::NOT_FOUND;
                    err_shorthand = "NOT_FOUND";
                } else {
                    error!("Error retrieving: {:?}", err);
                    status_code = StatusCode::INTERNAL_SERVER_ERROR;
                    err_shorthand = "INTERNAL_SERVER_ERROR";
                }

                Error::CustomError(
                    status_code,
                    ErrorDetail {
                        error: Some(err_shorthand.to_string()),
                        description: Some(err.to_string()),
                    },
                )
            })?;

    Ok(Redirect::temporary(&original))
}
