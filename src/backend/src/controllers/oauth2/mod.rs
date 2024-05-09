use axum_session::SessionNullPool;
use loco_oauth2::controllers::oauth2::{google_authorization_url, google_callback};
use loco_rs::prelude::*;

use crate::models::{
    _entities::{o_auth2_sessions, users},
    users::OAuth2UserProfile,
};

mod protected;

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/oauth2")
        // T is the session database pool
        .add("/google", get(google_authorization_url::<SessionNullPool>))
        // T is the OAuth2UserProfile, U is the user model, V is the OAuth2Session model, W is the
        // session database pool
        .add(
            "/google/callback",
            get(google_callback::<
                OAuth2UserProfile,
                users::Model,
                o_auth2_sessions::Model,
                SessionNullPool,
            >),
        )
        .add("/protected", get(protected::protected))
}
