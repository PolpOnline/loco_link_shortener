pub mod api;
pub mod retrieve;

use loco_rs::prelude::*;

pub fn routes() -> Routes {
    Routes::new().add("/x/:shortened", get(retrieve::retrieve))
}
