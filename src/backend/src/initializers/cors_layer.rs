use std::time::Duration;

use async_trait::async_trait;
use axum::Router as AxumRouter;
use loco_rs::{config, prelude::*};
use tower_http::cors;

pub struct CorsLayerInitializer;

#[async_trait]
impl Initializer for CorsLayerInitializer {
    fn name(&self) -> String {
        "cors_layer".to_string()
    }

    async fn after_routes(&self, mut router: AxumRouter, ctx: &AppContext) -> Result<AxumRouter> {
        let cors = ctx
            .config
            .server
            .middlewares
            .cors
            .as_ref()
            .filter(|cors| cors.enable)
            .map(Self::get_cors_middleware)
            .transpose()?;

        if let Some(cors) = cors {
            router = router.layer(cors)
        }

        Ok(router)
    }
}

impl CorsLayerInitializer {
    fn get_cors_middleware(config: &config::CorsMiddleware) -> Result<cors::CorsLayer> {
        let mut cors: cors::CorsLayer = cors::CorsLayer::permissive();

        if let Some(allow_origins) = &config.allow_origins {
            // testing CORS, assuming https://example.com in the allowlist:
            // $ curl -v --request OPTIONS 'localhost:3000/api/_ping' -H 'Origin: https://example.com' -H 'Access-Control-Request-Method: GET'
            // look for '< access-control-allow-origin: https://example.com' in response.
            // if it doesn't appear (test with a bogus domain), it is not allowed.
            let mut list = vec![];
            for origins in allow_origins {
                list.push(origins.parse()?);
            }
            cors = cors.allow_origin(list);
        }

        if let Some(allow_headers) = &config.allow_headers {
            let mut headers = vec![];
            for header in allow_headers {
                headers.push(header.parse()?);
            }
            cors = cors.allow_headers(headers);
        }

        if let Some(allow_methods) = &config.allow_methods {
            let mut methods = vec![];
            for method in allow_methods {
                methods.push(method.parse()?);
            }
            cors = cors.allow_methods(methods);
        }

        if let Some(max_age) = config.max_age {
            cors = cors.max_age(Duration::from_secs(max_age));
        }

        // FORCE ALLOW CREDENTIALS
        cors = cors.allow_credentials(true);

        Ok(cors)
    }
}
