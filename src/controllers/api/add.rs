use axum::http::StatusCode;
use loco_rs::{controller::ErrorDetail, prelude::*};
use rand::{distributions::Alphanumeric, Rng};
use tracing::error;

use crate::{
    common,
    controllers::api::add::types::{AddRequest, AddResponse},
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
        pub url: String,
        pub custom: Option<String>,
    }

    #[derive(Serialize)]
    pub struct AddResponse {
        pub shortened: String,
    }
}

pub async fn add(
    State(ctx): State<AppContext>,
    Json(params): Json<AddRequest>,
) -> Result<impl IntoResponse> {
    let settings = &ctx.config.settings.unwrap();
    let settings = common::settings::Settings::from_json(settings)?;

    if let Some(custom) = &params.custom {
        if custom.len() > settings.max_custom_length {
            return Err(Error::CustomError(
                StatusCode::BAD_REQUEST,
                ErrorDetail {
                    error: Some("CUSTOM_TOO_LONG".to_string()),
                    description: Some(format!(
                        "Custom shortened link is too long. Max length is {}",
                        settings.max_custom_length
                    )),
                },
            ));
        }
    }

    let shortened = generate_shortened(settings.shortened_length);

    links::Model::add(&ctx.db, params.url.as_str(), &shortened)
        .await
        .map_err(|err| {
            let status_code;
            let err_shorthand;

            if let AddError::InvalidUrl(ref _e) = err {
                status_code = StatusCode::BAD_REQUEST;
                err_shorthand = "INVALID_URL";
            } else {
                error!("Error adding: {:?}", err);
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

    Ok(Json(AddResponse { shortened }))
}

fn generate_shortened(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
