use anyhow::{Ok, Result};
use tokio::sync::watch::Sender;

use crate::packets::Packet;

pub async fn packet_listener(sender: Sender<Packet>) -> Result<()> {
    Ok(())
}
