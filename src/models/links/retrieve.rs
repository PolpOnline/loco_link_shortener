use loco_rs::{model::ModelError, prelude::*};
use sea_orm::{entity::prelude::*, DatabaseConnection};

pub use super::super::_entities::prelude::*;
use crate::models::_entities::links;

#[derive(thiserror::Error, Debug)]
pub enum RetrieveError {
    #[error("Entity not found")]
    NotFound,

    #[error(transparent)]
    ModelError(#[from] ModelError),
}

impl links::Model {
    /// Retrieves the original URL from the shortened URL and increases the
    /// click count
    pub async fn retrieve_original_and_increase_clicks_by_shortened<T: Into<String> + Send>(
        db: &DatabaseConnection,
        shortened: T,
    ) -> std::result::Result<String, RetrieveError> {
        let link = Links::find()
            .filter(links::Column::Shortened.eq(shortened.into()))
            .one(db)
            .await
            .map_err(ModelError::from)?
            .ok_or(RetrieveError::NotFound)?;

        let clicks = link.clicks + 1;
        let original = link.original.clone();

        let mut link = link.into_active_model();

        link.clicks = Set(clicks);

        link.update(db).await.map_err(ModelError::from)?;

        Ok(original)
    }
}
