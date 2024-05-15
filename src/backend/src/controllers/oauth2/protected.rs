use axum::{debug_handler, extract::State, response::IntoResponse};
use loco_oauth2::controllers::middleware::OAuth2CookieUser;
use loco_rs::{
    app::AppContext,
    controller::{format, unauthorized},
    prelude::*,
};
use serde::Serialize;
use tracing::error;

use crate::models::{
    _entities::{o_auth2_sessions, users},
    users::OAuth2UserProfile,
};

#[derive(Serialize)]
struct LoginResponse {
    email: String,
    token: String,
}

impl LoginResponse {
    fn new<T>(user: users::Model, token: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            email: user.email,
            token: token.into(),
        }
    }
}

pub async fn protected(
    State(ctx): State<AppContext>,
    // Extract the user from the Cookie via middleware
    user: OAuth2CookieUser<OAuth2UserProfile, users::Model, o_auth2_sessions::Model>,
) -> Result<impl IntoResponse> {
    let user = user.as_ref();
    let jwt_secret = ctx.config.get_jwt_config()?;

    let token = user
        .generate_jwt(&jwt_secret.secret, &jwt_secret.expiration)
        .or_else(|e| {
            error!("{:?}", e);
            unauthorized("unauthorized!")
        })?;

    let mut response = format::json(LoginResponse::new(user.clone(), token)).into_response();

    response
        .headers_mut()
        .append("Access-Control-Allow-Credentials", "true".parse().unwrap());

    Ok(response)
}
