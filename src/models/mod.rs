use sea_orm::ActiveModelBehavior;

pub mod _entities;
pub mod clicks;
pub mod links;

impl ActiveModelBehavior for _entities::links::ActiveModel {}
impl ActiveModelBehavior for _entities::clicks::ActiveModel {}
