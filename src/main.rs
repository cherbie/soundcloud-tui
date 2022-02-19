mod app;
mod components;
mod event;
mod views;

use crate::app::{render, App};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // main thread
    let mut app = App::new()?;

    render(&mut app).await?;

    Ok(())
}
