use loco_rs::prelude::*;
use serde::Serialize;

use crate::models::_entities::links;

#[derive(Serialize)]
pub struct ListResponse {
    links: Vec<Link>,
}

#[derive(Serialize)]
pub struct Link {
    original: String,
    shortened: String,
    created_at: DateTime,
}

impl ListResponse {
    pub fn new(links: Vec<Link>) -> ListResponse {
        ListResponse { links }
    }
}

impl From<links::Model> for Link {
    fn from(link: links::Model) -> Self {
        Link {
            original: link.original,
            shortened: link.shortened,
            created_at: link.created_at,
        }
    }
}

/// Retrieves all the links and their info
pub async fn list(State(ctx): State<AppContext>) -> Result<impl IntoResponse> {
    let link = links::Model::list(&ctx.db).await?;

    let links = link.into_iter().map(Link::from).collect();

    let res = ListResponse::new(links);

    Ok(Json(res))
}
