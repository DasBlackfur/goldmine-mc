use std::net::SocketAddr;

use anyhow::Result;
use declio::Decode;
use declio::Encode;
use mlua::Function;
use mlua::LuaSerdeExt;
use tokio::{net::UdpSocket, sync::watch::Sender};

use crate::constants::HANDSHAKE_COOKIE;
use crate::constants::HANDSHAKE_DATA;
use crate::constants::HANDSHAKE_DOUBLE_NULL;
use crate::constants::HANDSHAKE_FLAGS;
use crate::constants::HANDSHAKE_UNKNOWN;
use crate::constants::MAGIC;
use crate::constants::NULL_BYTE;
use crate::game_packets::Encapsulation;
use crate::game_packets::GamePacket;
use crate::{packets::Packet, Server};

pub async fn packet_listener(server: Server, _sender: Sender<String>) -> Result<()> {
    let socket = UdpSocket::bind(server.addr).await?;
    let mut buffer = Vec::with_capacity(1600);

    loop {
        buffer.clear();
        match listener_loop(&socket, &mut buffer, &server).await {
            Ok(_) => (),
            Err(err) => eprintln!("{:?}", err),
        }
    }
}

async fn listener_loop(socket: &UdpSocket, buffer: &mut Vec<u8>, server: &Server) -> Result<()> {
    let (len, sender_addr) = socket.recv_buf_from(buffer).await?;
    let packet = receive_packet(&buffer[..len], server)?;
    if let Packet::Custom { count, encapsulated: _ } = &packet {
        send_packet(buffer, socket, sender_addr, Packet::ACK { count: 0, single_value: true, packet_num: *count, packet_num_range: Default::default() }, server).await?;
    }
    if let Some(return_packet) = handle_packet(packet, server, &sender_addr)? {
        send_packet(buffer, socket, sender_addr, return_packet, server).await?;
    }
    Ok(())
}

fn receive_packet(mut buffer: &[u8], server: &Server) -> Result<Packet> {
    //println!("IN:  {:x?}", &buffer);
    let mut packet = Packet::decode((), &mut buffer)?;
    packet = execute_pl_callbacks(packet, server)?;
    Ok(packet)
}

async fn send_packet(
    buffer: &mut Vec<u8>,
    socket: &UdpSocket,
    addr: SocketAddr,
    mut packet: Packet,
    server: &Server,
) -> Result<()> {
    packet = execute_pl_callbacks(packet, server)?;
    buffer.clear();
    packet.encode((), buffer)?;
    //println!("OUT: {:x?}", &buffer);
    socket.send_to(buffer, addr).await?;
    Ok(())
}

fn execute_pl_callbacks(mut packet: Packet, server: &Server) -> Result<Packet> {
    for pl in server.registries.lock().borrow().pl_registry.values() {
        let lua_lock = server.lua.lock();
        let pl_callback: Function = lua_lock.registry_value(pl)?;
        packet = lua_lock.from_value(pl_callback.call(lua_lock.create_ser_userdata(packet)?)?)?;
    }
    Ok(packet)
}

fn handle_packet(
    packet: Packet,
    server: &Server,
    sender_addr: &SocketAddr,
) -> Result<Option<Packet>> {
    let return_packet = match packet {
        Packet::CSPingConnections { ping_id, magic: _ } => Some(Packet::SCPongConnections {
            ping_id,
            server_id: server.guid,
            magic: MAGIC,
            connection_string_len: server.server_name.len().try_into()?,
            connection_string: server.server_name.clone(),
        }),
        Packet::CSConnectionRequest1 {
            magic: _,
            raknet_version: _,
        } => Some(Packet::SCConnectionReply1 {
            magic: MAGIC,
            server_id: server.guid,
            null_byte: NULL_BYTE,
            mtu: 1447,
        }),
        Packet::CSConnectionRequest2 {
            magic: _,
            server_addr: _,
            server_port: _,
            mtu,
        } => Some(Packet::SCConnectionReply2 {
            magic: MAGIC,
            server_id: server.guid,
            client_port: sender_addr.port(),
            mtu,
            null_byte: NULL_BYTE,
        }),
        Packet::Custom {
            count,
            encapsulated,
        } => {
            let encapsulated_bytes = encapsulated.to_game_packet();
            println!("IN:  {:x?}", encapsulated_bytes);
            let packet = GamePacket::decode((), &mut encapsulated_bytes.as_slice())?;
            if let Some(return_packet) = handle_game_packet(packet, server)? {
                let mut return_packet_bytes = Vec::new();
                return_packet.encode((), &mut return_packet_bytes)?;
                println!("OUT: {:x?}", return_packet_bytes);
                Some(Packet::Custom {
                    count,
                    encapsulated: Encapsulation::Simple {
                        length: (return_packet_bytes.len() * 8) as u16,
                        game_packet: return_packet_bytes,
                    },
                })
            } else {
                None
            }
        }
        _ => None,
    };
    Ok(return_packet)
}

fn handle_game_packet(game_packet: GamePacket, server: &Server) -> Result<Option<GamePacket>> {
    let return_packet = match game_packet {
        GamePacket::CSClientConnect {
            client_id: _,
            session,
            unknown: _,
        } => Some(GamePacket::SCServerHandshake {
            cookie: HANDSHAKE_COOKIE,
            flags: HANDSHAKE_FLAGS,
            server_port: server.addr.port(),
            data: HANDSHAKE_DATA,
            unknown1: HANDSHAKE_DOUBLE_NULL,
            session,
            unknown2: HANDSHAKE_UNKNOWN,
        }),
        _ => None,
    };
    Ok(return_packet)
}

//fn handle_game_packet(
//    packet: GamePacket,
//    server: &Server,
//    sender_addr: &SocketAddr,
//) -> Result<Option<GamePacket>> {
//    let return_packet = match packet {
//        GamePacket::CSClientConnect(session_id) => Some(GamePacket::SCServerHandshake(session_id)),
//        _ => unreachable!(),
//    };
//
//    Ok(return_packet)
//}
