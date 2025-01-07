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
        client_ip_type: u8,
        client_ip: [u8; 4],
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
        #[declio(with = "encapsulation")]
        encapsulated: Vec<Encapsulation>,
    },
    #[declio(id = "0xC0")]
    ACK {
        #[declio(ctx = "ctx::Endian::Big")]
        count: u16,
        #[declio(with = "util::zero_one")]
        single_value: bool,
        #[declio(ctx = "ctx::Endian::Little")]
        packet_num: u24,
        #[declio(ctx = "ctx::Endian::Little", skip_if = "*single_value")]
        packet_num_range: u24,
    },
    #[declio(id = "0xA0")]
    NAK {
        #[declio(ctx = "ctx::Endian::Big")]
        count: u16,
        #[declio(with = "util::zero_one")]
        single_value: bool,
        #[declio(ctx = "ctx::Endian::Little")]
        packet_num: u24,
        #[declio(ctx = "ctx::Endian::Little", skip_if = "*single_value")]
        packet_num_range: u24,
    },
}

mod encapsulation {
    use declio::{Decode, Encode};

    use crate::game_packets::Encapsulation;

    pub fn encode<W>(
        encapsulated: &Vec<Encapsulation>,
        ctx: (),
        writer: &mut W,
    ) -> Result<(), declio::Error>
    where
        W: std::io::Write,
    {
        for packet in encapsulated {
            packet.encode(ctx, writer)?;
        }
        Ok(())
    }

    pub fn decode<R>(ctx: (), reader: &mut R) -> Result<Vec<Encapsulation>, declio::Error>
    where
        R: std::io::Read,
    {
        let mut encapsulated = Vec::new();
        while let Ok(packet) = Encapsulation::decode(ctx, reader) {
            encapsulated.push(packet);
        }
        Ok(encapsulated)
    }
}

impl UserData for Packet {
    fn add_methods<M: mlua::prelude::LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("toString", |_, packet, ()| Ok(format!("{:x?}", packet)))
    }
}
