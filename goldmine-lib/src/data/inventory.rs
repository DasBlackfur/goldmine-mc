pub type Inventory = Vec<Item>;

pub struct Item {
    pub id: u32,
    pub aux: u16,
    pub count: u8,
}
