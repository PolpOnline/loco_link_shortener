use axum::http::StatusCode;
use loco_rs::{controller::ErrorDetail, prelude::*};
use tracing::error;

use crate::{
    controllers::utils::get_user_from_jwt,
    models::{
        _entities::{clicks, links},
        links::find::FindError,
    },
    views::link_view::InfoLinkView,
};

/// Retrieves the info about the url
pub async fn info(
    jwt: auth::JWT,
    State(ctx): State<AppContext>,
    Path(shortened): Path<String>,
) -> Result<impl IntoResponse> {
    let user = get_user_from_jwt(&ctx, jwt).await?;

    let link = links::Model::find_by_shortened_where_user_id(&ctx.db, shortened, user.id)
        .await
        .map_err(|err| {
            let status_code;
            let err_shorthand;
            let mut err_desc = err.to_string();

            if let FindError::NotFound = err {
                status_code = StatusCode::NOT_FOUND;
                err_shorthand = "NOT_FOUND";
            } else {
                error!("Error getting info: {:?}", err);
                status_code = StatusCode::INTERNAL_SERVER_ERROR;
                err_shorthand = "INTERNAL_SERVER_ERROR";
                err_desc = "Internal server error".to_string();
            }

            Error::CustomError(status_code, ErrorDetail::new(err_shorthand, &err_desc))
        })?;

    let clicks = clicks::Model::get_clicks_by_id(&ctx.db, link.id).await?;

    let view = InfoLinkView::new(link, clicks);

    Ok(Json(view))
}
