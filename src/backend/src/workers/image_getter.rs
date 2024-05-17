use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::error;
use webpage::{Webpage, WebpageOptions};

use crate::models::{_entities::links, links::update_with_image::UpdateImageError};

pub struct LinkGetterWorker {
    pub ctx: AppContext,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct LinkGetterWorkerArgs {
    pub id: i32,
    pub url: String,
}

impl AppWorker<LinkGetterWorkerArgs> for LinkGetterWorker {
    fn build(ctx: &AppContext) -> Self {
        Self { ctx: ctx.clone() }
    }
}

#[async_trait]
impl worker::Worker<LinkGetterWorkerArgs> for LinkGetterWorker {
    async fn perform(&self, args: LinkGetterWorkerArgs) -> worker::Result<()> {
        let image = retrieve_og_image(&args.url);

        if let Some(image) = image {
            links::Model::update_with_image(&self.ctx.db, args.id, image)
                .await
                .unwrap_or_else(|e| match e {
                    UpdateImageError::NotFound => {
                        error!("Link not found");
                    }
                    _ => {
                        error!("Error updating link with image: {}", e);
                    }
                });
        }

        Ok(())
    }
}

pub fn retrieve_og_image(url: &str) -> Option<String> {
    let mut options = WebpageOptions::default();

    options.follow_location = false; // Disable redirects for security reasons

    let info = match Webpage::from_url(url, options) {
        Ok(info) => info,
        Err(_) => {
            return None;
        }
    };

    let image = info.html.opengraph.images;

    if image.is_empty() {
        return None;
    }

    Some(image[0].url.clone())
}
