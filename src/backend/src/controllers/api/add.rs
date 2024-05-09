use axum::http::StatusCode;
use loco_rs::{controller::ErrorDetail, prelude::*};
use rand::{distributions::Alphanumeric, Rng};
use tracing::error;

use crate::{
    common,
    controllers::{
        api::add::types::{AddRequest, AddResponse},
        utils::get_user_from_jwt,
    },
    models::{_entities::links, links::add::AddError},
};

mod validate_params {
    use validator::ValidationError;

    #[allow(dead_code)]
    pub fn validate_custom(custom: &Option<String>) -> Result<(), ValidationError> {
        if let Some(custom) = custom {
            if custom.chars().any(|c| !c.is_alphanumeric()) {
                return Err(ValidationError::new("CUSTOM_INVALID"));
            }
        }

        Ok(())
    }
}

pub mod types {
    use serde::{Deserialize, Serialize};
    use validator::Validate;

    #[derive(Deserialize, Validate)]
    pub struct AddRequest {
        pub name: String,
        pub url: String,
        pub custom: Option<String>,
    }

    #[derive(Serialize)]
    pub struct AddResponse {
        pub shortened: String,
    }
}

/// Adds a new link
pub async fn add(
    jwt: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<AddRequest>,
) -> Result<impl IntoResponse> {
    let user = get_user_from_jwt(&ctx, jwt).await?;

    let settings = &ctx.config.settings.unwrap();
    let settings = common::settings::Settings::from_json(settings)?;

    if let Some(custom) = &params.custom {
        if custom.len() > settings.max_custom_length {
            return Err(Error::CustomError(
                StatusCode::BAD_REQUEST,
                ErrorDetail::new(
                    "CUSTOM_TOO_LONG",
                    &format!(
                        "Custom shortened link is too long. Max length is {}",
                        settings.max_custom_length
                    ),
                ),
            ));
        }
    }

    let shortened = generate_shortened(settings.shortened_length);

    links::Model::add(
        &ctx.db,
        params.name.as_str(),
        params.url.as_str(),
        &shortened,
        user.id,
    )
    .await
    .map_err(|err| {
        let status_code;
        let err_shorthand;
        let mut err_desc = err.to_string();

        if let AddError::InvalidUrl(ref _e) = err {
            status_code = StatusCode::BAD_REQUEST;
            err_shorthand = "INVALID_URL";
        } else {
            error!("Error adding: {:?}", err);
            status_code = StatusCode::INTERNAL_SERVER_ERROR;
            err_shorthand = "INTERNAL_SERVER_ERROR";
            err_desc = "Internal server error".to_string();
        }

        Error::CustomError(status_code, ErrorDetail::new(err_shorthand, &err_desc))
    })?;

    Ok(Json(AddResponse { shortened }))
}

/// Generates a random string of a given length
fn generate_shortened(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
