use loco_rs::{model::ModelError, prelude::*};
use sea_orm::entity::prelude::*;

pub use super::super::_entities::prelude::*;
use crate::models::_entities::{clicks::ActiveModel as ClicksActiveModel, links};

#[derive(thiserror::Error, Debug)]
pub enum RetrieveError {
    #[error("Entity not found")]
    NotFound,

    #[error(transparent)]
    ModelError(#[from] ModelError),
}

impl links::Model {
    /// Retrieves the original URL from the shortened URL and increases the
    /// click count by adding a new click record
    pub async fn add_click_and_get_original(
        db: &DatabaseConnection,
        shortened: String,
        ip_address: String,
        user_agent: Option<String>,
    ) -> std::result::Result<String, RetrieveError> {
        let link = Links::find()
            .filter(links::Column::Shortened.eq(shortened))
            .one(db)
            .await
            .map_err(ModelError::from)?
            .ok_or(RetrieveError::NotFound)?;

        ClicksActiveModel {
            link_id: Set(link.id),
            clicked_at: Set(chrono::Utc::now().naive_local()),
            address: Set(ip_address),
            user_agent: Set(user_agent),
            ..Default::default()
        }
        .insert(db)
        .await
        .map_err(ModelError::from)?;

        Ok(link.original)
    }
}
