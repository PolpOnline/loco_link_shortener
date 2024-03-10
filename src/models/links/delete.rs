use loco_rs::{model::ModelError, prelude::*};
use sea_orm::{entity::prelude::*, DatabaseConnection};

pub use super::super::_entities::prelude::*;
use crate::models::_entities::links;

#[derive(thiserror::Error, Debug)]
pub enum DeleteError {
    #[error("Not found")]
    NotFound,

    #[error(transparent)]
    ModelError(#[from] ModelError),
}

impl links::Model {
    pub async fn delete<T: Into<String>>(
        db: &DatabaseConnection,
        shortened: T,
    ) -> std::result::Result<(), DeleteError> {
        let link = Links::find()
            .filter(links::Column::Shortened.eq(shortened.into()))
            .one(db)
            .await
            .map_err(ModelError::from)?
            .ok_or(DeleteError::NotFound)?;

        link.delete(db).await.map_err(ModelError::from)?;

        Ok(())
    }
}
