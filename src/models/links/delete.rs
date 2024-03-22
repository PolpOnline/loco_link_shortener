use loco_rs::{model::ModelError, prelude::*};
use sea_orm::entity::prelude::*;

pub use super::super::_entities::prelude::*;
use crate::models::{_entities::links, links::find::InfoError};

#[derive(thiserror::Error, Debug)]
pub enum DeleteError {
    #[error("Not found")]
    NotFound,

    #[error("User id not matching with shortened")]
    UserIdNotMatching,

    #[error(transparent)]
    ModelError(#[from] ModelError),
}

impl From<InfoError> for DeleteError {
    fn from(err: InfoError) -> Self {
        match err {
            InfoError::NotFound => DeleteError::NotFound,
            InfoError::ModelError(err) => DeleteError::ModelError(err),
        }
    }
}

impl links::Model {
    pub async fn delete<T: Into<String> + Send>(
        db: &DatabaseConnection,
        shortened: T,
        user_id: i32,
    ) -> std::result::Result<(), DeleteError> {
        let link = Self::find_by_shortened(db, shortened)
            .await
            .map_err(DeleteError::from)?;

        if link.user_id != user_id {
            return Err(DeleteError::UserIdNotMatching);
        }

        link.delete(db).await.map_err(ModelError::from)?;

        Ok(())
    }
}
