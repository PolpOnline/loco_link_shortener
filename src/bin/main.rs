use loco_link_shortener::app::App;
use loco_rs::cli;
use migration::Migrator;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    cli::main::<App, Migrator>().await
}
