use render::app;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    app::render().await?;

    Ok(())
}
