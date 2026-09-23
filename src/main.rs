use novellava::app;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    app().await?;
    Ok(())
}
