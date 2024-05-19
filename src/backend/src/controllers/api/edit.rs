use axum::http::StatusCode;
use loco_rs::{controller::ErrorDetail, model::ModelError, prelude::*};
use serde::Deserialize;
use tracing::error;

use crate::{
    common,
    controllers::{
        utils::{get_user_from_jwt, schedule_link_getter},
        validations::{custom::check_custom, url::check_url},
    },
    models::_entities::links,
};

#[derive(Deserialize)]
pub struct EditRequest {
    pub current_shortened: String,
    pub name: String,
    pub original: String,
    pub shortened: String,
}

#[derive(thiserror::Error, Debug)]
pub enum EditError {
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Invalid custom shortened link: {0}")]
    InvalidCustom(String),

    #[error("Not found")]
    NotFound,

    #[error(transparent)]
    InternalServerError(#[from] loco_rs::Error),

    #[error(transparent)]
    ModelError(#[from] ModelError),
}

/// Edits an existing link
pub async fn edit(
    jwt: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<EditRequest>,
) -> Result<impl IntoResponse> {
    let user = get_user_from_jwt(&ctx, jwt).await?;

    let settings = &ctx.clone().config.settings.unwrap();
    let settings = common::settings::Settings::from_json(settings)?;

    check_url(&params.original).map_err(|e| {
        let e: EditError = e.into();

        Error::CustomError(
            StatusCode::BAD_REQUEST,
            ErrorDetail::new("BAD_REQUEST", &e.to_string()),
        )
    })?;

    check_custom(&params.shortened, settings.max_custom_length).map_err(|e| {
        let e: EditError = e.into();

        Error::CustomError(
            StatusCode::BAD_REQUEST,
            ErrorDetail::new("BAD_REQUEST", &e.to_string()),
        )
    })?;

    let id = links::Model::edit_where_shortened_where_user_id(
        &ctx.db,
        user.id,
        &params.current_shortened,
        &params.name,
        &params.original,
        &params.shortened,
    )
    .await
    .map_err(|e| {
        let status_code;
        let err_shorthand;
        let mut err_desc = e.to_string();

        match e {
            EditError::NotFound => {
                status_code = StatusCode::NOT_FOUND;
                err_shorthand = "NOT_FOUND";
            }
            EditError::InvalidUrl(_) | EditError::InvalidCustom(_) => {
                status_code = StatusCode::BAD_REQUEST;
                err_shorthand = "BAD_REQUEST";
            }
            EditError::ModelError(ModelError::EntityAlreadyExists) => {
                status_code = StatusCode::CONFLICT;
                err_shorthand = "CONFLICT";
                err_desc = "Shortened link already exists".to_string();
            }
            EditError::InternalServerError(_) | EditError::ModelError(_) => {
                error!("Error editing link: {:?}", e);
                status_code = StatusCode::INTERNAL_SERVER_ERROR;
                err_shorthand = "INTERNAL_SERVER_ERROR";
                err_desc = "Internal server error".to_string();
            }
        };

        Error::CustomError(status_code, ErrorDetail::new(err_shorthand, &err_desc))
    })?;

    schedule_link_getter(&ctx, id, params.original).await?;

    Ok((StatusCode::OK, ""))
}
