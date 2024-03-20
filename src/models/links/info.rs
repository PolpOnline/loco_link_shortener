use loco_rs::{model::ModelError, prelude::*};
use sea_orm::entity::prelude::*;

use crate::models::_entities::links;

pub use super::super::_entities::prelude::*;

#[derive(thiserror::Error, Debug)]
pub enum InfoError {
    #[error("Not found")]
    NotFound,

    #[error(transparent)]
    ModelError(#[from] ModelError),
}

impl links::Model {
    pub async fn get_info_by_shortened<T: Into<String> + Send>(
        db: &DatabaseConnection,
        shortened: T,
    ) -> std::result::Result<links::Model, InfoError> {
        Links::find()
            .filter(links::Column::Shortened.eq(shortened.into()))
            .one(db)
            .await
            .map_err(ModelError::from)?
            .ok_or(InfoError::NotFound)
    }
}
