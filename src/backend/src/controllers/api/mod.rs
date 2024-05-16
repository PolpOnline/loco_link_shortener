use loco_rs::prelude::*;

pub mod add;
pub mod delete;
pub mod info;
pub mod list;

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/")
        .add("/add", post(add::add))
        .add("/delete", delete(delete::delete))
        .add("/info/:shortened", get(info::info))
        .add("/list", get(list::list))
}
