use std::net::IpAddr;

use axum::http::{HeaderMap, StatusCode};
use loco_rs::{app::AppContext, controller::ErrorDetail, prelude::auth, Error};
use tracing::error;
use uuid::Uuid;

use crate::models::_entities::users;

/// Checks if the user is authenticated and gets the user from the database
pub async fn get_user_from_jwt(ctx: &AppContext, jwt: auth::JWT) -> loco_rs::Result<users::Model> {
    let claims = jwt.claims;
    let pid = claims.pid;

    let pid = Uuid::parse_str(&pid).map_err(|e| {
        error!("{:?}", e);
        Error::CustomError(
            StatusCode::BAD_REQUEST,
            ErrorDetail::new("INVALID_PID", "Invalid user ID"),
        )
    })?;

    let user = users::Model::find_by_pid(&ctx.db, pid)
        .await
        .map_err(|e| {
            error!("{:?}", e);
            Error::CustomError(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorDetail::new("INTERNAL_SERVER_ERROR", "Database error"),
            )
        })?
        .ok_or_else(|| {
            Error::CustomError(
                StatusCode::NOT_FOUND,
                ErrorDetail::new("USER_NOT_FOUND", "User not found"),
            )
        })?;

    Ok(user)
}

/// Get the IP address from the request headers (railway.app includes the real
/// IP in the "x-Envoy-external-Address" or "x-forwarded-for" headers)
pub fn get_ip(ip_address: &IpAddr, headers: &HeaderMap) -> String {
    if let Some(ip) = headers
        .get("x-Envoy-external-Address")
        .and_then(|header| header.to_str().ok())
    {
        return ip.to_string();
    }

    if let Some(ip) = headers
        .get("x-forwarded-for")
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.split(',').last())
    {
        return ip.to_string();
    }

    ip_address.to_canonical().to_string()
}
