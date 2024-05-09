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
        name: T,
        original: T,
        shortened: T,
        user_id: i32,
    ) -> std::result::Result<(), AddError> {
        ActiveModel {
            name: Set(name.into()),
            original: Set(original.into()),
            shortened: Set(shortened.into()),
            user_id: Set(user_id),
            ..Default::default()
        }
        .insert(db)
        .await
        .map_err(ModelError::from)?;

        Ok(())
    }
}
