pub mod _entities;
pub mod links;

use sea_orm::ActiveModelBehavior;

impl ActiveModelBehavior for _entities::links::ActiveModel {}
impl ActiveModelBehavior for _entities::clicks::ActiveModel {}