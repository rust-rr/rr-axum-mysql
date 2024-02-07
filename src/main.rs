use anyhow::Result;

mod db;

#[tokio::main]
async fn main() -> Result<()> {
    db::dev_init().await;
    Ok(())
}
