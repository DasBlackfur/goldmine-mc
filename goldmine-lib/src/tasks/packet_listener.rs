use std::net::SocketAddr;

use anyhow::Result;
use mlua::Function;
use mlua::LuaSerdeExt;
use tokio::{net::UdpSocket, sync::watch::Sender};

use crate::constants::RAKNET_VERSION;
use crate::game_packets::GamePacket;
use crate::{packets::Packet, Server};

pub async fn packet_listener(server: Server, _sender: Sender<Packet>) -> Result<()> {
    let socket = UdpSocket::bind(server.addr.clone()).await?;
    let mut buffer = [0; 1024];

    loop {
        let (len, sender_addr) = socket.recv_from(&mut buffer).await?;
        let packet = receive_packet(&buffer[..len], &server)?;
        if let Some(return_packet) = handle_packet(packet, &server, &sender_addr)? {
            send_packet(&socket, sender_addr, return_packet, &server).await?;
        }
    }
}

fn receive_packet(buffer: &[u8], server: &Server) -> Result<Packet> {
    println!("{:x?}", buffer);
    let mut packet = Packet::from_bytes(buffer)?;
    packet = execute_pl_callbacks(packet, server)?;
    Ok(packet)
}

async fn send_packet(
    socket: &UdpSocket,
    addr: SocketAddr,
    mut packet: Packet,
    server: &Server,
) -> Result<()> {
    packet = execute_pl_callbacks(packet, server)?;
    println!("{:x?}", &packet.as_bytes()?);
    socket.send_to(&packet.as_bytes()?, addr).await?;
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
        Packet::NoOP => None,
        Packet::CSPingConnections(ping_id) => {
            let return_packet = Packet::SCPingOpenConnections(
                ping_id,
                server.guid,
                format!("MCCPP;Demo;{}", server.server_name),
            );
            Some(return_packet)
        }
        Packet::CSConnectionRequest1(raknet_version, packet_length) => {
            if raknet_version == RAKNET_VERSION {
                Some(Packet::SCConnectionReply1(
                    server.guid,
                    packet_length as u16,
                ))
            } else {
                Some(Packet::SCIncompatibleProtocol(RAKNET_VERSION, server.guid))
            }
        }
        Packet::CSConnectionRequest2(mtu_size) => Some(Packet::SCConnectionReply2(
            server.guid,
            sender_addr.port(),
            mtu_size,
        )),
        Packet::Encapsulated(count, game_packet) => {
            handle_game_packet(game_packet, server, sender_addr)?
                .map(|game_return_packet| Packet::Encapsulated(count, game_return_packet))
        }
        _ => unreachable!(),
    };
    Ok(return_packet)
}

fn handle_game_packet(
    packet: GamePacket,
    server: &Server,
    sender_addr: &SocketAddr,
) -> Result<Option<GamePacket>> {
    let return_packet = match packet {
        GamePacket::CSClientConnect(session_id) => Some(GamePacket::SCServerHandshake(session_id)),
        _ => unreachable!(),
    };

    Ok(return_packet)
}
