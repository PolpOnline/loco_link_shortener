use axum::{async_trait, Router as AxumRouter};
use axum_client_ip::SecureClientIpSource;
use loco_rs::prelude::*;

pub struct IPGetterInitializer;

#[async_trait]
impl Initializer for IPGetterInitializer {
    fn name(&self) -> String {
        "ip_getter".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, _ctx: &AppContext) -> Result<AxumRouter> {
        let app = router.layer(SecureClientIpSource::ConnectInfo.into_extension());

        Ok(app)
    }
}
