use std::mem;

use serde::{Deserialize, Serialize};

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
pub struct u24(u32);

impl u24 {
    #[inline(always)]
    pub const fn from_be(x: Self) -> Self {
        #[cfg(target_endian = "big")]
        {
            x
        }
        #[cfg(not(target_endian = "big"))]
        {
            u24(x.0.swap_bytes())
        }
    }

    #[inline]
    pub fn from_be_bytes(bytes: [u8; 3]) -> Self {
        Self::from_be(u24::from_ne_bytes(bytes))
    }

    #[inline]
    pub fn from_ne_bytes(input_bytes: [u8; 3]) -> u24 {
        // SAFETY: integers are plain old datatypes so we can always transmute to them
        let mut bytes: [u8; 4] = [0; 4];
        bytes[1..].copy_from_slice(&input_bytes);
        unsafe { mem::transmute(bytes) }
    }
}
