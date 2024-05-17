use loco_rs::model::ModelError;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait};
use thiserror::Error;

use crate::models::_entities::{links, prelude::Links};

#[derive(Error, Debug)]
pub enum UpdateImageError {
    #[error(transparent)]
    ModelError(#[from] ModelError),

    #[error("Link not found")]
    NotFound,
}

impl links::Model {
    pub async fn update_with_image<T: Into<String> + Send>(
        db: &DatabaseConnection,
        id: i32,
        image: T,
    ) -> Result<(), UpdateImageError> {
        let link = Links::find_by_id(id)
            .one(db)
            .await
            .map_err(ModelError::from)?;

        let mut link: links::ActiveModel = link.ok_or(UpdateImageError::NotFound)?.into();

        link.image = Set(Some(image.into()));

        link.update(db).await.map_err(ModelError::from)?;

        Ok(())
    }
}
