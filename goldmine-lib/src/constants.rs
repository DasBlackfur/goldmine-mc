pub const MAGIC: [u8; 16] = 0x00ffff00fefefefefdfdfdfd12345678_u128.to_be_bytes();
pub const RAKNET_VERSION: u8 = 120;
pub const OUT_OF_DATA: &str = "Not enough data in packet";
pub const HANDSHAKE_DATA: [u8; 70] = [
    0, 0, 4, 245, 255, 255, 245, 0, 0, 4, 255, 255, 255, 255, 0, 0, 4, 255, 255, 255, 255, 0, 0, 4,
    255, 255, 255, 255, 0, 0, 4, 255, 255, 255, 255, 0, 0, 4, 255, 255, 255, 255, 0, 0, 4, 255,
    255, 255, 255, 0, 0, 4, 255, 255, 255, 255, 0, 0, 4, 255, 255, 255, 255, 0, 0, 4, 255, 255,
    255, 255,
];
