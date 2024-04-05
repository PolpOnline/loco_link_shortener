use axum::{
    http::{HeaderMap, StatusCode},
    response::Redirect,
};
use axum_client_ip::SecureClientIp;
use loco_rs::{controller::ErrorDetail, prelude::*};
use tracing::error;

use crate::{
    models::{_entities::links, links::retrieve::RetrieveError},
    utils::get_ip,
};

/// Retrieves the original URL from the shortened URL and redirects to it
pub async fn retrieve(
    State(ctx): State<AppContext>,
    Path(shortened): Path<String>,
    secure_ip: SecureClientIp,
    headers: HeaderMap,
) -> Result<impl IntoResponse> {
    let ip_address = get_ip(&secure_ip, &headers);

    let original = links::Model::add_click_and_get_original(&ctx.db, shortened, ip_address)
        .await
        .map_err(|err| {
            let status_code;
            let err_shorthand;
            let mut err_desc = err.to_string();

            if let RetrieveError::NotFound = err {
                status_code = StatusCode::NOT_FOUND;
                err_shorthand = "NOT_FOUND";
            } else {
                error!("Error retrieving: {:?}", err);
                status_code = StatusCode::INTERNAL_SERVER_ERROR;
                err_shorthand = "INTERNAL_SERVER_ERROR";
                err_desc = "Internal server error".to_string();
            }

            Error::CustomError(status_code, ErrorDetail::new(err_shorthand, &err_desc))
        })?;

    Ok(Redirect::temporary(&original))
}
