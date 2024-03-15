use loco_rs::{model::ModelResult, prelude::*};
use sea_orm::DatabaseConnection;

pub use super::super::_entities::prelude::*;
use crate::models::_entities::links;

impl links::Model {
    pub async fn list(db: &DatabaseConnection) -> ModelResult<Vec<Self>> {
        Ok(Links::find().all(db).await?)
    }
}
