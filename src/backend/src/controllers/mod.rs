use loco_rs::prelude::*;

pub mod api;
pub mod custom_headers;
pub mod oauth2;
pub mod retrieve;
pub mod utils;
pub mod validations;

pub fn routes() -> Routes {
    Routes::new().add("/x/:shortened", get(retrieve::retrieve))
}
