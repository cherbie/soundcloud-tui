mod components;
mod domain;
mod event;
mod utils;

#[cfg(test)]
mod tests;

use crate::domain::app::{render, App};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // main thread
    let mut app = App::new()?;

    render(&mut app).await?;

    Ok(())
}
