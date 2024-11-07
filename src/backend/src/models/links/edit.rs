use loco_rs::model::ModelError;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

use crate::{controllers::api::edit::EditError, models::_entities::links};

impl links::Model {
    pub async fn edit_where_shortened_where_user_id<T: Into<String> + Send>(
        db: &DatabaseConnection,
        user_id: i32,
        current_shortened: T,
        name: T,
        original: T,
        shortened: T,
    ) -> Result<i32, EditError> {
        let shortened = shortened.into();
        let current_shortened = current_shortened.into();

        if links::Entity::find()
            .filter(links::Column::Shortened.eq(&shortened))
            .one(db)
            .await
            .map_err(ModelError::from)?
            .is_some()
            && shortened != current_shortened
        {
            return Err(EditError::from(ModelError::EntityAlreadyExists));
        }

        let link = links::Entity::find()
            .filter(links::Column::Shortened.eq(current_shortened))
            .filter(links::Column::UserId.eq(user_id))
            .one(db)
            .await
            .map_err(ModelError::from)?;

        let mut link: links::ActiveModel = link.ok_or(EditError::NotFound)?.into();

        link.name = Set(name.into());
        link.original = Set(original.into());
        link.shortened = Set(shortened);

        let updated = link.update(db).await.map_err(ModelError::from)?;

        Ok(updated.id)
    }
}
