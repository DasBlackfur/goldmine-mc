use anyhow::Result;
use mlua::Function;
use mlua::LuaSerdeExt;
use tokio::{net::UdpSocket, sync::watch::Sender};

use crate::{packets::Packet, Server};

pub async fn packet_listener(server: Server, _sender: Sender<Packet>) -> Result<()> {
    let socket = UdpSocket::bind(server.addr).await?;
    let mut buffer = [0; 1024];

    loop {
        let (_len, sender_addr) = socket.recv_from(&mut buffer).await?;
        match Packet::from_bytes(&buffer) {
            Ok(mut packet) => {
                println!("{:?}", packet);
                for pl in server.registries.lock().borrow().pl_registry.values() {
                    let lua_lock = server.lua.lock();
                    let pl_callback: Function = lua_lock.registry_value(pl)?;
                    packet = lua_lock
                        .from_value(pl_callback.call(lua_lock.create_ser_userdata(packet)?).unwrap()).unwrap();
                }
                match packet {
                    Packet::CSPingConnections(ping_id) => {
                        socket
                            .send_to(
                                &Packet::SCPingOpenConnections(
                                    ping_id,
                                    server.guid,
                                    format!("MCCPP;Demo;{}", server.server_name),
                                )
                                .as_bytes()?,
                                sender_addr,
                            )
                            .await?;
                    }
                    _ => unreachable!(),
                }
            }
            Err(e) => {
                println!("{}", e);
                println!("Offending packet:\n{:?}", &buffer);
            }
        }
    }
}
