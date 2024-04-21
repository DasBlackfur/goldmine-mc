use anyhow::{Context, Error, Result};
use mlua::UserData;
use serde::{Deserialize, Serialize};

pub const MAGIC: [u8; 16] = 0x00ffff00fefefefefdfdfdfd12345678_u128.to_be_bytes();

#[derive(Serialize, Deserialize, Debug)]
pub enum Packet {
    NoOP,
    /// ping_id
    CSPingConnections(u64),
    /// ping_id, server_id, connection_string
    SCPingOpenConnections(u64, u64, String),
}

impl UserData for Packet {
    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("toString", |_, packet, ()| Ok(format!("{:?}", packet)))
    }
}

impl Packet {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let packet_id = bytes.first().context("Packet doesn't contain an ID")?;
        match packet_id {
            0x02 => Ok(Self::CSPingConnections(u64::from_be_bytes(
                bytes
                    .get(1..=8)
                    .context("Not enough data in packet")?
                    .try_into()?,
            ))),
            _ => Err(Error::msg(format!("No packet with ID {:x}", packet_id))),
        }
    }

    pub fn as_bytes(&self) -> Result<Vec<u8>> {
        match self {
            Packet::SCPingOpenConnections(ping_id, server_id, server_str) => Ok({
                let mut packet: Vec<u8> = vec![0x1C];
                packet.extend_from_slice(&ping_id.to_be_bytes());
                packet.extend_from_slice(&server_id.to_be_bytes());
                packet.extend_from_slice(&MAGIC);
                packet.extend_from_slice(&(server_str.len() as u16).to_be_bytes());
                packet.extend_from_slice(server_str.as_bytes());
                packet
            }),
            _ => Err(Error::msg(format!(
                "Packet {:?} can't be sent from the server",
                self
            ))),
        }
    }
}
