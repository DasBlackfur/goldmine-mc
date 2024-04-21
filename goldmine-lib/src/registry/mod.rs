use std::collections::{hash_map::Values, HashMap};

use anyhow::{Context, Result};

use self::{api_module::ApiModuleRegistry, lua_mod::LuaModRegistry, packet_listener::PacketListenerRegistry};

pub mod api_module;
pub mod packet_listener;
pub mod lua_mod;

pub struct Registry<V> {
    internal: HashMap<String, V>,
}

impl<V> Registry<V> {
    pub fn new() -> Registry<V> {
        Registry {
            internal: HashMap::new(),
        }
    }

    pub fn register(&mut self, key: &str, value: V) {
        self.internal.insert(key.to_owned(), value);
    }

    pub fn get(&self, key: &str) -> Result<&V> {
        self.internal
            .get(key)
            .context(format!("No such key {}", key))
    }

    pub fn values(&self) -> Values<String, V> {
        self.internal.values()
    }
}

impl<V> Default for Registry<V> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Registries {
    pub pl_registry: PacketListenerRegistry,
    pub api_registry: ApiModuleRegistry,
    pub lm_registry: LuaModRegistry,
}

impl Registries {
    pub fn new() -> Self {
        Self {
            pl_registry: PacketListenerRegistry::new(),
            api_registry: ApiModuleRegistry::new(),
            lm_registry: LuaModRegistry::new()
        }
    }
}

impl Default for Registries {
    fn default() -> Self {
        Self::new()
    }
}
