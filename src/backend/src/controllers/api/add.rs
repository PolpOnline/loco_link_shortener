use axum::http::StatusCode;
use loco_rs::{controller::ErrorDetail, model::ModelError, prelude::*};
use rand::{Rng, distr::Alphanumeric};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::error;
use witty_phrase_generator::WPGen;

use crate::{
    common,
    controllers::{
        utils::{get_user_from_jwt, schedule_link_getter},
        validations::{custom::check_custom, url::check_url},
    },
    models::_entities::links,
};

#[derive(Deserialize)]
pub struct AddRequest {
    pub name: Option<String>,
    pub url: String,
    pub custom: Option<String>,
}

#[derive(Serialize)]
pub struct AddResponse {
    pub shortened: String,
}

#[derive(Error, Debug)]
pub enum AddError {
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Invalid custom shortened link: {0}")]
    InvalidCustom(String),

    #[error(transparent)]
    InternalServerError(#[from] loco_rs::Error),

    #[error(transparent)]
    ModelError(#[from] ModelError),
}

/// Adds a new link
pub async fn add(
    jwt: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<AddRequest>,
) -> Result<impl IntoResponse> {
    let user = get_user_from_jwt(&ctx, jwt).await?;

    let settings = &ctx.clone().config.settings.unwrap();
    let settings = common::settings::Settings::from_json(settings)?;

    let name = match &params.name {
        Some(name) => name.clone(),
        None => generate_witty_name().unwrap_or_else(|e| {
            error!("Could not generate witty name {}", e);
            "Link".to_string()
        }),
    };

    let shortened = match &params.custom {
        Some(custom) => {
            check_custom(custom, settings.max_custom_length).map_err(|e| {
                let e: AddError = e.into();

                Error::CustomError(
                    StatusCode::BAD_REQUEST,
                    ErrorDetail::new("INVALID_CUSTOM", &e.to_string()),
                )
            })?;
            custom.clone()
        }
        None => generate_shortened(settings.shortened_length),
    };

    check_url(&params.url).map_err(|e| {
        let e: AddError = e.into();

        Error::CustomError(
            StatusCode::BAD_REQUEST,
            ErrorDetail::new("INVALID_URL", &e.to_string()),
        )
    })?;

    let id = links::Model::add(
        &ctx.db,
        name.as_str(),
        params.url.as_str(),
        &shortened,
        user.id,
    )
    .await
    .map_err(|err| {
        let status_code;
        let err_shorthand;
        let mut err_desc = err.to_string();

        match err {
            AddError::InvalidUrl(ref _e) => {
                status_code = StatusCode::BAD_REQUEST;
                err_shorthand = "INVALID_URL";
            }
            AddError::ModelError(ModelError::EntityAlreadyExists) => {
                status_code = StatusCode::CONFLICT;
                err_shorthand = "ENTITY_EXISTS";
                err_desc = "Shortened link already exists".to_string();
            }
            _ => {
                error!("Error adding: {:?}", err);
                status_code = StatusCode::INTERNAL_SERVER_ERROR;
                err_shorthand = "INTERNAL_SERVER_ERROR";
                err_desc = "Internal server error".to_string();
            }
        }

        Error::CustomError(status_code, ErrorDetail::new(err_shorthand, &err_desc))
    })?;

    schedule_link_getter(&ctx, id, params.url.clone()).await?;

    Ok(Json(AddResponse { shortened }))
}

/// Generates a random string of a given length
fn generate_shortened(length: usize) -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

fn generate_witty_name() -> Result<String> {
    let wp_gen = WPGen::new();

    Ok(wp_gen
        .generic(
            3,        // words per phrase
            1,        // phrases
            None,     // minimum length
            Some(25), // maximum length
            None,     // maximum word length
            None,
        )
        .ok_or(Error::Message(
            "Could not generate a witty phrase".to_string(),
        ))?[0]
        .join(" "))
}
