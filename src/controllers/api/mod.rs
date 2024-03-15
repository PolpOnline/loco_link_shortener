pub mod add;
pub mod delete;
pub mod info;
pub mod list;

use loco_rs::prelude::*;

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/")
        .add("/add", post(add::add))
        .add("/delete", delete(delete::delete))
        .add("/info", get(info::info))
        .add("/list", get(list::list))
}
