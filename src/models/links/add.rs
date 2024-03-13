use loco_rs::{model::ModelError, prelude::*};
use sea_orm::DatabaseConnection;

pub use super::super::_entities::prelude::*;
use crate::models::_entities::{links, links::ActiveModel};

#[derive(thiserror::Error, Debug)]
pub enum AddError {
    #[error("Invalid URL: {0}")]
    InvalidUrl(#[from] validator::ValidationErrors),

    #[error(transparent)]
    ModelError(#[from] ModelError),
}

impl links::Model {
    pub async fn add<T: Into<String> + Send>(
        db: &DatabaseConnection,
        original: T,
        shortened: T,
    ) -> std::result::Result<(), AddError> {
        ActiveModel {
            original: Set(original.into()),
            shortened: Set(shortened.into()),
            ..Default::default()
        }
        .insert(db)
        .await
        .map_err(ModelError::from)?;

        Ok(())
    }
}
