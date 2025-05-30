use loco_rs::prelude::*;

pub mod api;
pub mod custom_headers;
mod healthcheck;
pub mod oauth2;
pub mod retrieve;
pub mod system_info;
pub mod utils;
pub mod validations;

pub fn routes() -> Routes {
    Routes::new()
        .add("/x/:shortened", get(retrieve::retrieve))
        .add("/sys_info", get(system_info::sys_info))
        .add("/healthcheck", get(healthcheck::healthcheck))
}
