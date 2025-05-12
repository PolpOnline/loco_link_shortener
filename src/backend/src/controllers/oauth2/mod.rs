use std::fmt::Debug;

use axum::{Extension, response::Response};
use axum_session::{DatabasePool, Session, SessionNullPool};
use loco_oauth2::{
    OAuth2ClientStore,
    controllers::oauth2::{get_authorization_url, google_callback},
};
use loco_rs::prelude::*;

use crate::models::{
    _entities::{o_auth2_sessions, users},
    users::OAuth2UserProfile,
};

mod protected;

pub async fn google_authorization_url<T: DatabasePool + Clone + Debug + Sync + Send + 'static>(
    session: Session<T>,
    Extension(oauth2_store): Extension<OAuth2ClientStore>,
) -> Result<Response<String>> {
    let mut client = oauth2_store
        .get_authorization_code_client("google")
        .await
        .map_err(|e| {
            tracing::error!("Error getting client: {:?}", e);
            Error::InternalServerError
        })?;
    let auth_url = get_authorization_url(session, &mut client).await;
    let mut response = Response::new(auth_url);
    // add header to response
    response
        .headers_mut()
        .append("Access-Control-Allow-Credentials", "true".parse().unwrap());
    Ok(response)
}

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
