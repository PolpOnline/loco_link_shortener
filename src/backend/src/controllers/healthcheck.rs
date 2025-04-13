use axum::response::IntoResponse;

pub async fn healthcheck() -> loco_rs::Result<impl IntoResponse> {
    Ok("OK".into_response())
}
