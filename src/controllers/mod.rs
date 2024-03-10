pub mod api;
pub mod retrieve;

use axum::response::Redirect;
use loco_rs::prelude::*;

pub fn routes() -> Routes {
    Routes::new()
        .add("/", get(dash))
        .add("/:shortened", get(retrieve::retrieve))
}

/// Redirects to the dashboard
async fn dash() -> impl IntoResponse {
    Redirect::permanent("/dash")
}
