use axum::http::StatusCode;
use loco_rs::{controller::ErrorDetail, prelude::*};
use serde::Deserialize;
use tracing::error;

use crate::{
    models::{
        _entities::{clicks, links},
        links::info::InfoError,
    },
    views::link_view::InfoLinkView,
};

#[derive(Deserialize)]
pub struct InfoRequest {
    pub shortened: String,
}

/// Retrieves the info about the url
pub async fn info(
    State(ctx): State<AppContext>,
    Json(params): Json<InfoRequest>,
) -> Result<impl IntoResponse> {
    let link = links::Model::get_info_by_shortened(&ctx.db, &params.shortened)
        .await
        .map_err(|err| {
            let status_code;
            let err_shorthand;

            if let InfoError::NotFound = err {
                status_code = StatusCode::NOT_FOUND;
                err_shorthand = "NOT_FOUND";
            } else {
                error!("Error getting info: {:?}", err);
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

    let clicks = clicks::Model::get_clicks_by_id(&ctx.db, link.id).await?;

    let view = InfoLinkView::new(link, clicks);

    Ok(Json(view))
}
