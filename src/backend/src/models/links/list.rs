use loco_rs::{model::ModelResult, prelude::*};
use sea_orm::{ColumnTrait, DatabaseConnection, QueryFilter, QueryOrder};

pub use super::super::_entities::prelude::*;
use crate::models::_entities::links;

impl links::Model {
    pub async fn list_where_user_id(
        db: &DatabaseConnection,
        user_id: i32,
    ) -> ModelResult<Vec<Self>> {
        Ok(Links::find()
            .filter(links::Column::UserId.eq(user_id))
            .order_by_desc(links::Column::CreatedAt)
            .all(db)
            .await?)
    }
}
