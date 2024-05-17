use loco_rs::{model::ModelError, prelude::*};
use sea_orm::entity::prelude::*;

pub use super::super::_entities::prelude::*;
use crate::models::_entities::links;

#[derive(thiserror::Error, Debug)]
pub enum FindError {
    #[error("Link not found")]
    NotFound,

    #[error(transparent)]
    ModelError(#[from] ModelError),
}

impl links::Model {
    pub async fn find_by_shortened<T: Into<String> + Send>(
        db: &DatabaseConnection,
        shortened: T,
    ) -> std::result::Result<links::Model, FindError> {
        Links::find()
            .filter(links::Column::Shortened.eq(shortened.into()))
            .one(db)
            .await
            .map_err(ModelError::from)?
            .ok_or(FindError::NotFound)
    }

    pub async fn find_by_shortened_where_user_id<T: Into<String> + Send>(
        db: &DatabaseConnection,
        shortened: T,
        user_id: i32,
    ) -> std::result::Result<links::Model, FindError> {
        Links::find()
            .filter(links::Column::Shortened.eq(shortened.into()))
            .filter(links::Column::UserId.eq(user_id))
            .one(db)
            .await
            .map_err(ModelError::from)?
            .ok_or(FindError::NotFound)
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> std::result::Result<links::Model, FindError> {
        Links::find()
            .filter(links::Column::Id.eq(id))
            .one(db)
            .await
            .map_err(ModelError::from)?
            .ok_or(FindError::NotFound)
    }
}
