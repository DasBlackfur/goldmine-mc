#![allow(non_camel_case_types)]
use declio::magic_bytes;
use serde::{Deserialize, Serialize};

magic_bytes! {
    #[derive(Serialize, Deserialize, Debug)]
    pub MAGIC(&0x00ffff00fefefefefdfdfdfd12345678_u128.to_be_bytes());
    #[derive(Serialize, Deserialize, Debug)]
    pub NULL_BYTE(&[0_u8]);
    #[derive(Serialize, Deserialize, Debug)]
    pub HANDSHAKE_COOKIE(&0x043f57fe_u32.to_be_bytes());
    #[derive(Serialize, Deserialize, Debug)]
    pub HANDSHAKE_FLAGS(&[0xcd_u8]);
    #[derive(Serialize, Deserialize, Debug)]
    pub HANDSHAKE_DATA(&([
        0, 0, 4, 245, 255, 255, 245, 0, 0, 4, 255, 255, 255, 255, 0, 0, 4, 255, 255, 255, 255, 0, 0, 4,
        255, 255, 255, 255, 0, 0, 4, 255, 255, 255, 255, 0, 0, 4, 255, 255, 255, 255, 0, 0, 4, 255,
        255, 255, 255, 0, 0, 4, 255, 255, 255, 255, 0, 0, 4, 255, 255, 255, 255, 0, 0, 4, 255, 255,
        255, 255,
    ] as [u8; 70]));
    #[derive(Serialize, Deserialize, Debug)]
    pub HANDSHAKE_DOUBLE_NULL(&0_u16.to_ne_bytes());
    #[derive(Serialize, Deserialize, Debug)]
    pub HANDSHAKE_UNKNOWN(&([0x00, 0x00, 0x00, 0x00, 0x04, 0x44, 0x0b, 0xa9] as [u8; 8]));
}
