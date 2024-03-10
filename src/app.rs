use std::path::Path;

use async_trait::async_trait;
use loco_rs::{
    app::{AppContext, Hooks, Initializer},
    boot::{create_app, BootResult, StartMode},
    controller::AppRoutes,
    db::truncate_table,
    environment::Environment,
    task::Tasks,
    worker::Processor,
    Result,
};
use migration::Migrator;
use sea_orm::DatabaseConnection;

use crate::{controllers, models::_entities::prelude::*};

pub struct App;

#[async_trait]
impl Hooks for App {
    fn app_version() -> String {
        format!(
            "{} ({})",
            env!("CARGO_PKG_VERSION"),
            option_env!("BUILD_SHA")
                .or(option_env!("GITHUB_SHA"))
                .unwrap_or("dev")
        )
    }

    fn app_name() -> &'static str {
        env!("CARGO_CRATE_NAME")
    }

    async fn boot(mode: StartMode, environment: &Environment) -> Result<BootResult> {
        create_app::<Self, Migrator>(mode, environment).await
    }

    async fn initializers(_ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
        Ok(vec![])
    }

    fn routes(_ctx: &AppContext) -> AppRoutes {
        AppRoutes::with_default_routes()
            .add_route(controllers::routes())
            .add_route(controllers::api::routes())
    }

    fn connect_workers(_p: &mut Processor, _ctx: &AppContext) {}

    fn register_tasks(_tasks: &mut Tasks) {}

    async fn truncate(db: &DatabaseConnection) -> Result<()> {
        truncate_table(db, Links).await?;
        Ok(())
    }

    async fn seed(_db: &DatabaseConnection, _base: &Path) -> Result<()> {
        Ok(())
    }
}
