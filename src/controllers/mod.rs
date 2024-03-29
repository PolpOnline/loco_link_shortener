use loco_rs::prelude::*;

pub mod api;
pub mod oauth2;
pub mod retrieve;
pub mod utils;

pub fn routes() -> Routes {
    Routes::new().add("/x/:shortened", get(retrieve::retrieve))
}
