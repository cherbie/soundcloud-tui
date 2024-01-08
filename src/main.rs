mod api;
mod components;
mod domain;
mod event;
mod utils;

use crate::domain::app;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    app::render().await?;

    Ok(())
}
