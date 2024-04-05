use axum::{
    http::{HeaderMap, StatusCode},
    response::Redirect,
};
use axum_client_ip::InsecureClientIp;
use axum_extra::TypedHeader;
use headers::UserAgent;
use loco_rs::{controller::ErrorDetail, prelude::*};
use tracing::error;

use super::utils::get_ip;
use crate::models::{_entities::links, links::retrieve::RetrieveError};

/// Retrieves the original URL from the shortened URL and redirects to it
pub async fn retrieve(
    State(ctx): State<AppContext>,
    Path(shortened): Path<String>,
    address: InsecureClientIp,
    headers: HeaderMap,
    user_agent: Option<TypedHeader<UserAgent>>,
) -> Result<impl IntoResponse> {
    let ip_address = get_ip(&address.0, &headers);
    let user_agent = user_agent.map(|ua| ua.0.to_string());

    let original =
        links::Model::add_click_and_get_original(&ctx.db, shortened, ip_address, user_agent)
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
