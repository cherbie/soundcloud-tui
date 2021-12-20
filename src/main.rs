mod app;
mod event;

use crate::app::{render, App};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // main thread
    let mut app = App::default();

    render(&mut app).await?;

    Ok(())
}
