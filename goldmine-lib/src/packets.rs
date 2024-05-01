use anyhow::{Context, Error, Result};
use mlua::UserData;
use serde::{Deserialize, Serialize};

use crate::{game_packets::GamePacket, u24::u24};

pub const MAGIC: [u8; 16] = 0x00ffff00fefefefefdfdfdfd12345678_u128.to_be_bytes();
pub const RAKNET_VERSION: u8 = 120;

#[derive(Serialize, Deserialize, Debug)]
pub enum Packet {
    NoOP,
    /// ping_id
    CSPingConnections(u64),
    /// ping_id, server_id, connection_string
    SCPingOpenConnections(u64, u64, String),
    /// raknet_version, packet_length
    CSConnectionRequest1(u8, usize),
    /// server_id, mtu_size
    SCConnectionReply1(u64, u16),
    /// raknet_version, server_id
    SCIncompatibleProtocol(u8, u64),
    /// mtu_size
    CSConnectionRequest2(u16),
    /// server_id, udp_port, mtu_size
    SCConnectionReply2(u64, u16, u16),
    /// count, game_packet
    Encapsulated(u24, GamePacket),
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
            0x05 => Ok(Self::CSConnectionRequest1(
                *bytes.get(16).context("Not enough data in packet")?,
                bytes.len(),
            )),
            0x07 => Ok(Self::CSConnectionRequest2(u16::from_be_bytes(
                bytes
                    .get(24..=25)
                    .context("Not enough data in packet")?
                    .try_into()?,
            ))),
            0x80..=0x8F => Ok(Self::Encapsulated(
                u24::from_be_bytes(
                    bytes
                        .get(1..=3)
                        .context("Not enough data in packet")?
                        .try_into()?,
                ),
                GamePacket::NoOP,
            )),
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
            Packet::SCConnectionReply1(server_id, mtu_size) => Ok({
                let mut packet: Vec<u8> = vec![0x06];
                packet.extend_from_slice(&MAGIC);
                packet.extend_from_slice(&server_id.to_be_bytes());
                packet.extend_from_slice(&[0]);
                packet.extend_from_slice(&mtu_size.to_be_bytes());
                packet
            }),
            Packet::SCIncompatibleProtocol(raknet_version, server_id) => Ok({
                let mut packet: Vec<u8> = vec![0x1A];
                packet.extend_from_slice(&[*raknet_version]);
                packet.extend_from_slice(&MAGIC);
                packet.extend_from_slice(&server_id.to_be_bytes());
                packet
            }),
            Packet::SCConnectionReply2(server_id, udp_port, mtu_size) => Ok({
                let mut packet: Vec<u8> = vec![0x08];
                packet.extend_from_slice(&MAGIC);
                packet.extend_from_slice(&server_id.to_be_bytes());
                packet.extend_from_slice(&udp_port.to_be_bytes());
                packet.extend_from_slice(&mtu_size.to_be_bytes());
                packet.extend_from_slice(&[0]);
                packet
            }),
            _ => Err(Error::msg(format!(
                "Packet {:?} can't be sent from the server",
                self
            ))),
        }
    }
}
