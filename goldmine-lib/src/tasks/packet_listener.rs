use std::net::SocketAddr;
use std::net::SocketAddrV4;

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
use crate::constants::SERVER_VERSION;
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

fn get_connection_id(server: &Server, sender_addr: &SocketAddr) -> Result<u64> {
    let mut connections = server.connections.lock();
    let connection_id : u64;
    if !connections.contains_left(&sender_addr) {
        let mut counter = server.unique_connection_id.lock();
        connection_id = *counter;
        *counter += 1;
        println!("New connection id {}", connection_id);
        connections.insert(*sender_addr, connection_id);
    } else {
        if let Some(id) = connections.get_by_left(&sender_addr) {
            connection_id = *id;
            println!("Existing connection id {}", connection_id);
        } else {
            panic!("This should never execute. What did you do?!?");
        }
    }
    Ok(connection_id)
}

async fn listener_loop(socket: &UdpSocket, buffer: &mut Vec<u8>, server: &Server) -> Result<()> {
    let (len, sender_addr) = socket.recv_buf_from(buffer).await?;
    if let SocketAddr::V4(socket_addr) = sender_addr {
        let connection_id : u64 = get_connection_id(server, &sender_addr)?;
        let packet = receive_packet(&buffer[..len], server, connection_id)?;
        if let Packet::Custom {
            count,
            encapsulated: _,
        } = &packet
        {
            send_packet(
                buffer,
                socket,
                connection_id,
                Packet::ACK {
                    count: 1,
                    single_value: true,
                    packet_num: *count,
                    packet_num_range: Default::default(),
                },
                server,
            )
            .await?;
        }
        if let Some(return_packets) = handle_packet(packet, server, &socket_addr)? {
            for return_packet in return_packets {
                send_packet(buffer, socket, connection_id, return_packet, server).await?;
            }
        }
        Ok(())
    } else {
        Err(anyhow::Error::msg("No IPv6 support"))
    }
}

fn receive_packet(mut buffer: &[u8], server: &Server, connection_id: u64) -> Result<Packet> {
    //println!("IN:  {:x?}", &buffer);
    let mut packet = Packet::decode((), &mut buffer)?;
    packet = execute_pl_callbacks(packet, server, true, connection_id)?;
    Ok(packet)
}

async fn send_packet(
    buffer: &mut Vec<u8>,
    socket: &UdpSocket,
    connection_id: u64,
    mut packet: Packet,
    server: &Server,
) -> Result<()> {
    if let Some(addr) = server.connections.lock().get_by_right(&connection_id) {
        packet = execute_pl_callbacks(packet, server, false, connection_id)?;
        buffer.clear();
        packet.encode((), buffer)?;
        //println!("OUT: {:x?}", &buffer);
        socket.send_to(buffer, addr).await?;
        Ok(())
    } else {
        panic!("Unknown connection_id {}", connection_id);
    }
    
}

fn execute_pl_callbacks(mut packet: Packet, server: &Server, inbound: bool, connection_id: u64) -> Result<Packet> {
    for pl in server.registries.lock().pl_registry.values() {
        let lua_lock = server.lua.lock();
        let pl_callback: Function = lua_lock.registry_value(pl)?;
        packet = lua_lock.from_value(pl_callback.call((lua_lock.create_ser_userdata(packet)?, inbound, connection_id))?)?;
    }
    Ok(packet)
}

fn handle_packet(
    packet: Packet,
    server: &Server,
    sender_addr: &SocketAddrV4,
) -> Result<Option<Vec<Packet>>> {
    let return_packet = match packet {
        Packet::CSPingConnections { ping_id, magic: _ } => Some(vec![Packet::SCPongConnections {
            ping_id,
            server_id: server.guid,
            magic: MAGIC,
            connection_string_len: server.server_name.len().try_into()?,
            connection_string: server.server_name.clone(),
        }]),
        Packet::CSConnectionRequest1 {
            magic: _,
            raknet_version: _,
        } => Some(vec![Packet::SCConnectionReply1 {
            magic: MAGIC,
            server_id: server.guid,
            null_byte: NULL_BYTE,
            mtu: 1447,
        }]),
        Packet::CSConnectionRequest2 {
            magic: _,
            server_addr: _,
            server_port: _,
            mtu,
        } => Some(vec![Packet::SCConnectionReply2 {
            magic: MAGIC,
            server_id: server.guid,
            client_ip_type: 0x04,
            client_ip: sender_addr.ip().octets(),
            client_port: sender_addr.port(),
            mtu,
            null_byte: NULL_BYTE,
        }]),
        Packet::Custom {
            count,
            encapsulated,
        } => {
            let mut returns = Vec::new();
            for encapsulation in encapsulated {
                let encapsulated_bytes = encapsulation.to_game_packet();
                println!("IN:  {:x?}", encapsulated_bytes);
                let packet = GamePacket::decode((), &mut encapsulated_bytes.as_slice())?;
                if let Some(return_packets) = handle_game_packet(packet, server)? {
                    return_packets.iter().for_each(|x| {
                        let mut x_bytes = Vec::new();
                        if x.encode((), &mut x_bytes).is_ok() {
                            returns.push(Encapsulation::Simple {
                                length: (x_bytes.len() * 8) as u16,
                                game_packet: x_bytes,
                            })
                        };
                    });
                }
            }
            if !returns.is_empty() {
                Some(vec![Packet::Custom {
                    count,
                    encapsulated: returns,
                }])
            } else {
                None
            }
        }
        _ => None,
    };
    Ok(return_packet)
}

fn handle_game_packet(game_packet: GamePacket, server: &Server) -> Result<Option<Vec<GamePacket>>> {
    let return_packet = match game_packet {
        GamePacket::CSPing { ping_id } => Some(vec![GamePacket::SCPong { ping_id, pong_id: 0 }]),
        GamePacket::CSClientConnect {
            client_id: _,
            session,
            unknown: _,
        } => Some(vec![GamePacket::SCServerHandshake {
            cookie: HANDSHAKE_COOKIE,
            flags: HANDSHAKE_FLAGS,
            server_port: server.addr.port(),
            data: HANDSHAKE_DATA,
            unknown1: HANDSHAKE_DOUBLE_NULL,
            session,
            unknown2: HANDSHAKE_UNKNOWN,
        }]),
        GamePacket::CSLogin {
            username_len: _,
            username: _,
            proto1,
            proto2: _,
        } => {
            let login_status = GamePacket::SCLoginStatus {
                status: match proto1.cmp(&SERVER_VERSION) {
                    std::cmp::Ordering::Less => 1,
                    std::cmp::Ordering::Equal => 0,
                    std::cmp::Ordering::Greater => 2,
                },
            };
            let player = server.add_player();
            let start_game = GamePacket::SCStartGame {
                seed: server.get_seed(),
                worldgen_version: 4,
                gamemode: server.get_gamemode(),
                entity_id: player.id,
                pos_x: player.pos.0,
                pos_y: player.pos.1,
                pos_z: player.pos.2,
            };
            Some(vec![login_status, start_game])
        }
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
