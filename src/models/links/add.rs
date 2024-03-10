use loco_rs::{model::ModelError, prelude::*};
use sea_orm::DatabaseConnection;

pub use super::super::_entities::prelude::*;
use crate::models::_entities::{links, links::ActiveModel};

#[derive(thiserror::Error, Debug)]
pub enum AddError {
    #[error("Invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),

    #[error(transparent)]
    ModelError(#[from] ModelError),
}

impl links::Model {
    pub async fn add<T: Into<String>>(
        db: &DatabaseConnection,
        original: T,
        shortened: T,
    ) -> std::result::Result<(), AddError> {
        // Validate the URL by parsing it
        let url = url::Url::parse(original.into().as_str()).map_err(AddError::from)?;

        ActiveModel {
            original: Set(url.as_str().to_string()),
            shortened: Set(shortened.into()),
            ..Default::default()
        }
        .insert(db)
        .await
        .map_err(ModelError::from)?;

        Ok(())
    }
}
