use anyhow::Result;

use goldmine_lib::Server;

#[tokio::main]
async fn main() -> Result<()> {
    let mut server = Server::new("0.0.0.0:19132", "example_mod/mod.lua")?;
    server.execute().await?;
    Ok(())
}
