use loco_rs::{model::ModelError, prelude::*};
use regex::Regex;
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
        let original = original.into();

        check_url(&original)?;

        let link = ActiveModel {
            name: Set(name.into()),
            original: Set(original),
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

const URL_REGEX: &str = r"https?:\/\/(www\.)?[-a-zA-Z0-9@:%._+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_+.~#?&/=]*)";

fn check_url(url: &str) -> std::result::Result<(), AddError> {
    if url.is_empty() {
        return Err(AddError::InvalidUrl("URL cannot be empty".to_string()));
    }

    let re = Regex::new(URL_REGEX).unwrap();

    if !re.is_match(url) {
        return Err(AddError::InvalidUrl(
            "Please insert a valid URL".to_string(),
        ));
    }

    Ok(())
}
