use crate::{data::EntityData, Server};

impl Server {
    pub fn add_player(&self) -> EntityData {
        let player = EntityData {
            id: rand::random(),
            pos: (0.0, 0.0, 0.0),
            rot: (0.0, 0.0, 0.0),
        };
        self.data.lock().borrow_mut().entities.push(player.clone());
        player
    }

    pub fn get_seed(&self) -> u32 {
        self.data.lock().borrow().seed
    }

    pub fn get_gamemode(&self) -> u32 {
        self.data.lock().borrow().gamemode
    }
}
