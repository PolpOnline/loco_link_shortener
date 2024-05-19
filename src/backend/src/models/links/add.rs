use loco_rs::{model::ModelError, prelude::*};
use sea_orm::DatabaseConnection;

pub use super::super::_entities::prelude::*;
use crate::{
    controllers::api::add::AddError,
    models::_entities::{links, links::ActiveModel},
};

impl links::Model {
    pub async fn add<T: Into<String> + Send>(
        db: &DatabaseConnection,
        name: T,
        original: T,
        shortened: T,
        user_id: i32,
    ) -> std::result::Result<i32, AddError> {
        let link = ActiveModel {
            name: Set(name.into()),
            original: Set(original.into()),
            shortened: Set(shortened.into()),
            user_id: Set(user_id),
            ..Default::default()
        };

        let res = Links::insert(link)
            .exec(db)
            .await
            .map_err(ModelError::from)?;

        Ok(res.last_insert_id)
    }
}
