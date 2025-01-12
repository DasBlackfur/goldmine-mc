use crate::error::{LuaInitGlobalSnafu, LuaInitSnafu, Result};

// Registry
// add(String, V) -> add(String, V, priority: 0)
// add(String, V, priority: i8)

use mlua::Lua;
use snafu::ResultExt;

// Raw packet listener -> function (packet, client, direction) -> packet|nil //packet.rs
// Packet listener -> function (packet, client, direction) -> packet|nil //game_packet.rs
// Event listener -> function(event) -> event|nil
pub mod registry;

// API
// @goldmine -> server info
// @goldmine/packets -> types, serialization, deserialization, sendPacket(packet, client_id)
// @goldmine/entity -> position, rotation, getClientId(entity_id) -> client_id
// @goldmine/world -> blocks, entities
// @goldmine/attachment -> getAttachments(pos/entity_id/block_id+aux/entity_type, type) -> attachment_id,
//                         getAttachmentTypes(pos/entity_id/block_id+aux/entity_type) -> Vec<attachment_type>,
//                         setAttachment(pos/entity_id/block_id+aux/entity_type, type, data)
//                         getBlocks(attachment_type, chunk_x, chunk_z) -> Vec<(x,y,z)
//                         getBlocks(attachment_type) -> Vec<(x,y,z)>
//                         getEntities(attachment_type) -> Vec<entity_id>
pub mod module;

// Attachments
// many-to-many entity_id <-> attachment_type
// BiMap (entity_id, attachment_type) <-> attachment_id
// many-to-many (x,y,z) <-> attachment_type
// BiMap (x,y,z, attachment_type) <-> attachment_id
// HashMap attachment_id -> data

// attachment_id -> pos, type
// type -> all pos
// pos -> types -> attachment_id

// many-to-many -> sql table
// BiMap -> sql table
// HashMap -> sql table -> |attachment_id|string|

pub mod require;

pub fn init_lua(lua: &Lua) -> Result<()> {
    lua.globals()
        .set(
            "goldmine",
            module::goldmine_module(lua).context(LuaInitSnafu)?,
        )
        .context(LuaInitGlobalSnafu)?;

    Ok(())
}
