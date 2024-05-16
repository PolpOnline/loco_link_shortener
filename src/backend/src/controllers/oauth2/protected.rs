use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
};
use loco_oauth2::controllers::middleware::OAuth2CookieUser;
use loco_rs::{app::AppContext, controller::unauthorized, prelude::*};
use serde::Serialize;
use tracing::error;

use crate::{
    common,
    models::{
        _entities::{o_auth2_sessions, users},
        users::OAuth2UserProfile,
    },
};

pub async fn protected(
    State(ctx): State<AppContext>,
    // Extract the user from the Cookie via middleware
    user: OAuth2CookieUser<OAuth2UserProfile, users::Model, o_auth2_sessions::Model>,
) -> Result<impl IntoResponse> {
    let user = user.as_ref();
    let jwt_secret = ctx.config.get_jwt_config().cloned()?;
    let settings = &ctx.config.settings.unwrap();
    let settings = common::settings::Settings::from_json(settings)?;

    let token = user
        .generate_jwt(&jwt_secret.secret, &jwt_secret.expiration)
        .or_else(|e| {
            error!("{:?}", e);
            unauthorized("unauthorized!")
        })?;

    // let mut response = format::json(LoginResponse::new(user.clone(),
    // token)).into_response();
    //
    // response
    //     .headers_mut()
    //     .append("Access-Control-Allow-Credentials", "true".parse().unwrap());
    //
    // Ok(response)

    let frontend_url = settings.frontend_url;

    // Redirect to the frontend with the token
    let redirect_url = format!("{}/login/callback?t={}", frontend_url, token);

    Ok(Redirect::to(&redirect_url))
}
