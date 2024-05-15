use axum::{async_trait, Extension, Router as AxumRouter};
use loco_oauth2::{config::OAuth2Config, OAuth2ClientStore};
use loco_rs::prelude::*;

pub struct OAuth2StoreInitializer;

#[async_trait]
impl Initializer for OAuth2StoreInitializer {
    fn name(&self) -> String {
        "oauth2-store".to_string()
    }
    async fn after_routes(&self, router: AxumRouter, ctx: &AppContext) -> Result<AxumRouter> {
        // Get all the settings from the config
        let settings = ctx
            .config
            .settings
            .clone()
            .ok_or_else(|| Error::Message("Settings config not configured".to_string()))?;
        // Get the oauth2 config in json format
        let oauth2_config_value = settings
            .get("oauth2")
            .ok_or(Error::Message("OAuth2 config not found".to_string()))?
            .clone();
        // Convert the oauth2 config json to OAuth2Config
        let oauth2_config: OAuth2Config = oauth2_config_value.try_into().map_err(|e| {
            tracing::error!(error = ?e, "Could not convert oauth2 config");
            Error::Message("Could not convert oauth2 config".to_string())
        })?;
        // Create the OAuth2ClientStore
        let oauth2_store = OAuth2ClientStore::new(oauth2_config).map_err(|e| {
            tracing::error!(error = ?e, "Could not create oauth2 store");
            Error::Message("Could not create oauth2 store".to_string())
        })?;
        // Add the OAuth2ClientStore to the AxumRouter as an extension
        Ok(router.layer(Extension(oauth2_store)))
    }
}
