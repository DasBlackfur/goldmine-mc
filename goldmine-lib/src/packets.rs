use declio::{ctx, util, Decode, Encode};
use mlua::UserData;
use serde::{Deserialize, Serialize};

use crate::{
    constants::{MAGIC, NULL_BYTE},
    game_packets::Encapsulation,
    u24::u24,
};

//pub const MAGIC: [u8; 16] = 0x00ffff00fefefefefdfdfdfd12345678_u128.to_be_bytes();
pub const RAKNET_VERSION: u8 = 120;

#[derive(Serialize, Deserialize, Debug, Encode, Decode)]
#[declio(id_type = "u8")]
pub enum Packet {
    #[declio(id = "0x02")]
    CSPingConnections {
        #[declio(ctx = "ctx::Endian::Big")]
        ping_id: u64,
        magic: MAGIC,
    },
    #[declio(id = "0x1C")]
    SCPongConnections {
        #[declio(ctx = "ctx::Endian::Big")]
        ping_id: u64,
        #[declio(ctx = "ctx::Endian::Big")]
        server_id: u64,
        magic: MAGIC,
        #[declio(ctx = "ctx::Endian::Big")]
        connection_string_len: u16,
        #[declio(with = "util::utf8", ctx = "ctx::Len((*connection_string_len).into())")]
        connection_string: String,
    },
    #[declio(id = "0x05")]
    CSConnectionRequest1 { magic: MAGIC, raknet_version: u8 },
    #[declio(id = "0x06")]
    SCConnectionReply1 {
        magic: MAGIC,
        #[declio(ctx = "ctx::Endian::Big")]
        server_id: u64,
        null_byte: NULL_BYTE,
        #[declio(ctx = "ctx::Endian::Big")]
        mtu: u16,
    },
    #[declio(id = "0x07")]
    CSConnectionRequest2 {
        magic: MAGIC,
        server_addr: [u8; 5],
        #[declio(ctx = "ctx::Endian::Big")]
        server_port: u16,
        #[declio(ctx = "ctx::Endian::Big")]
        mtu: u16,
    },
    #[declio(id = "0x08")]
    SCConnectionReply2 {
        magic: MAGIC,
        #[declio(ctx = "ctx::Endian::Big")]
        server_id: u64,
        #[declio(ctx = "ctx::Endian::Big")]
        client_port: u16,
        #[declio(ctx = "ctx::Endian::Big")]
        mtu: u16,
        null_byte: NULL_BYTE,
    },
    #[declio(id = "0x84")]
    Custom {
        #[declio(ctx = "ctx::Endian::Little")]
        count: u24,
        encapsulated: Encapsulation,
    },
    #[declio(id = "0xC0")]
    ACK {
        #[declio(ctx = "ctx::Endian::Big")]
        count: u16,
        #[declio(with ="util::zero_one")]
        single_value: bool,
        #[declio(ctx = "ctx::Endian::Little")]
        packet_num: u24,
        #[declio(ctx = "ctx::Endian::Little", skip_if="*single_value")]
        packet_num_range: u24
    },
    #[declio(id = "0xA0")]
    NAK {
        #[declio(ctx = "ctx::Endian::Big")]
        count: u16,
        #[declio(with ="util::zero_one")]
        single_value: bool,
        #[declio(ctx = "ctx::Endian::Little")]
        packet_num: u24,
        #[declio(ctx = "ctx::Endian::Little", skip_if="*single_value")]
        packet_num_range: u24
    }
}

impl UserData for Packet {
    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("toString", |_, packet, ()| Ok(format!("{:x?}", packet)))
    }
}

//impl Packet {
//    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
//        let packet_id = bytes.first().context("Packet doesn't contain an ID")?;
//        match packet_id {
//            0x02 => Ok(Self::CSPingConnections(u64::from_be_bytes(
//                bytes
//                    .get(1..=8)
//                    .context("Not enough data in packet")?
//                    .try_into()?,
//            ))),
//            0x05 => Ok(Self::CSConnectionRequest1(
//                *bytes.get(16).context("Not enough data in packet")?,
//                bytes.len(),
//            )),
//            0x07 => Ok(Self::CSConnectionRequest2(u16::from_be_bytes(
//                bytes
//                    .get(24..=25)
//                    .context("Not enough data in packet")?
//                    .try_into()?,
//            ))),
//            0x80..=0x8F => Ok(Self::Encapsulated(
//                u24::from_be_bytes(
//                    bytes
//                        .get(1..=3)
//                        .context("Not enough data in packet")?
//                        .try_into()?,
//                ),
//                GamePacket::NoOP,
//            )),
//            _ => Err(Error::msg(format!("No packet with ID {:x}", packet_id))),
//        }
//    }
//
//    pub fn as_bytes(&self) -> Result<Vec<u8>> {
//        match self {
//            Packet::SCPingOpenConnections(ping_id, server_id, server_str) => Ok({
//                let mut packet: Vec<u8> = vec![0x1C];
//                packet.extend_from_slice(&ping_id.to_be_bytes());
//                packet.extend_from_slice(&server_id.to_be_bytes());
//                packet.extend_from_slice(&MAGIC);
//                packet.extend_from_slice(&(server_str.len() as u16).to_be_bytes());
//                packet.extend_from_slice(server_str.as_bytes());
//                packet
//            }),
//            Packet::SCConnectionReply1(server_id, mtu_size) => Ok({
//                let mut packet: Vec<u8> = vec![0x06];
//                packet.extend_from_slice(&MAGIC);
//                packet.extend_from_slice(&server_id.to_be_bytes());
//                packet.extend_from_slice(&[0]);
//                packet.extend_from_slice(&mtu_size.to_be_bytes());
//                packet
//            }),
//            Packet::SCIncompatibleProtocol(raknet_version, server_id) => Ok({
//                let mut packet: Vec<u8> = vec![0x1A];
//                packet.extend_from_slice(&[*raknet_version]);
//                packet.extend_from_slice(&MAGIC);
//                packet.extend_from_slice(&server_id.to_be_bytes());
//                packet
//            }),
//            Packet::SCConnectionReply2(server_id, udp_port, mtu_size) => Ok({
//                let mut packet: Vec<u8> = vec![0x08];
//                packet.extend_from_slice(&MAGIC);
//                packet.extend_from_slice(&server_id.to_be_bytes());
//                packet.extend_from_slice(&udp_port.to_be_bytes());
//                packet.extend_from_slice(&mtu_size.to_be_bytes());
//                packet.extend_from_slice(&[0]);
//                packet
//            }),
//            _ => Err(Error::msg(format!(
//                "Packet {:?} can't be sent from the server",
//                self
//            ))),
//        }
//    }
//}
//
