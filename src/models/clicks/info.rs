use loco_rs::{
    model::{ModelError, ModelResult},
    prelude::*,
};
use sea_orm::entity::prelude::*;

use crate::models::_entities::clicks;

pub use super::super::_entities::prelude::*;

#[derive(thiserror::Error, Debug)]
pub enum InfoError {
    #[error("Not found")]
    NotFound,

    #[error(transparent)]
    ModelError(#[from] ModelError),
}

impl clicks::Model {
    pub async fn get_clicks_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> ModelResult<Vec<clicks::Model>> {
        Ok(Clicks::find()
            .filter(clicks::Column::LinkId.eq(id))
            .all(db)
            .await?)
    }
}
