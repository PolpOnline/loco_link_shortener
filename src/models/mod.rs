use sea_orm::ActiveModelBehavior;

pub mod _entities;
pub mod clicks;
pub mod links;
pub mod o_auth2_sessions;
pub mod users;

impl ActiveModelBehavior for _entities::o_auth2_sessions::ActiveModel {}

impl ActiveModelBehavior for _entities::users::ActiveModel {}
impl ActiveModelBehavior for _entities::links::ActiveModel {}
impl ActiveModelBehavior for _entities::clicks::ActiveModel {}
