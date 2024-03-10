pub mod add;
pub mod delete;
pub mod info;

use loco_rs::prelude::*;

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/")
        .add("/add", post(add::add))
        .add("/delete", delete(delete::delete))
        .add("/info", get(info::info))
}