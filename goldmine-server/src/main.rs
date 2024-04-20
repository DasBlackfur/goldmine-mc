use anyhow::Result;

use goldmine_lib::Server;

#[tokio::main]
async fn main() -> Result<()> {
    let mut server = Server::new()?;
    server.execute().await?;
    Ok(())
}
