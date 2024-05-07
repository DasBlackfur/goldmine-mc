use anyhow::{Context, Error, Result};
use serde::{Deserialize, Serialize};

use crate::constants::{HANDSHAKE_DATA, OUT_OF_DATA};

#[derive(Serialize, Deserialize, Debug)]
pub enum EncapsulationType {
    Simple,
    ExtendedCount,
    ExtendedFull,
}

impl EncapsulationType {
    pub fn from_byte(byte: u8) -> Result<Self> {
        match byte {
            0x00 => Ok(Self::Simple),
            0x40 => Ok(Self::ExtendedCount),
            0x60 => Ok(Self::ExtendedFull),
            t => Err(Error::msg(format!("Unknown encapsulation type: {:x}", t))),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GamePacket {
    NoOP,
    /// session_id
    CSClientConnect(u64),
    /// session_id
    SCServerHandshake(u64),
}

impl GamePacket {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let enc_type =
            EncapsulationType::from_byte(*bytes.get(4).context("Not enough data in  packet")?)?;
        println!("{:?}", enc_type);
        let mut game_bytes = Vec::new();
        match enc_type {
            EncapsulationType::Simple => {
                game_bytes.extend_from_slice(bytes.get(7..).context(OUT_OF_DATA)?)
            }
            EncapsulationType::ExtendedCount => {
                game_bytes.extend_from_slice(bytes.get(10..).context(OUT_OF_DATA)?)
            }
            EncapsulationType::ExtendedFull => {
                game_bytes.extend_from_slice(bytes.get(14..).context(OUT_OF_DATA)?)
            }
        }
        let packet_id = game_bytes.first().context("Packet doesn't contain an ID")?;
        match packet_id {
            0x09 => Ok(Self::CSClientConnect(u64::from_be_bytes(
                game_bytes.get(9..=16).context(OUT_OF_DATA)?.try_into()?,
            ))),
            _ => Err(Error::msg(format!(
                "No game packet with ID {:x}",
                packet_id
            ))),
        }
    }

    pub fn as_bytes(&self) -> Result<Vec<u8>> {
        let mut packet: Vec<u8> = vec![0x00, 0xff, 0xff];
        match self {
            Self::SCServerHandshake(session_id) => Ok({
                packet.extend_from_slice(&[0x10]);
                packet.extend_from_slice(&[0x04, 0x3f, 0x57, 0xfe]);
                packet.extend_from_slice(&[0xcd]);
                packet.extend_from_slice(&[0x00; 2]);
                packet.extend_from_slice(&HANDSHAKE_DATA);
                packet.extend_from_slice(&[0x00; 2]);
                packet.extend_from_slice(&session_id.to_be_bytes());
                packet.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x04, 0x44, 0x0b, 0xa9]);
                let len = (((packet.len() - 3) * 2) as u16).to_be_bytes();
                packet[1] = len[0];
                packet[2] = len[1];
                packet
            }),
            _ => Err(Error::msg(format!(
                "Game-packet {:?} can't be sent from the server",
                self
            ))),
        }
    }
}
