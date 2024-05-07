use std::mem;

use serde::{Deserialize, Serialize};

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
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

    #[inline(always)]
    pub const fn to_be(self) -> Self {
        // or not to be?
        #[cfg(target_endian = "big")]
        {
            self
        }
        #[cfg(not(target_endian = "big"))]
        {
            u24(self.0.swap_bytes())
        }
    }

    #[inline]
    pub fn from_be_bytes(bytes: [u8; 3]) -> Self {
        Self::from_be(u24::from_ne_bytes(bytes))
    }

    #[inline]
    pub fn to_be_bytes(self) -> [u8; 3] {
        self.to_be().to_ne_bytes()
    }

    #[inline]
    pub fn from_ne_bytes(input_bytes: [u8; 3]) -> u24 {
        // SAFETY: integers are plain old datatypes so we can always transmute to them
        let mut bytes: [u8; 4] = [0; 4];
        bytes[1..].copy_from_slice(&input_bytes);
        unsafe { mem::transmute(bytes) }
    }

    #[inline]
    pub fn to_ne_bytes(self) -> [u8; 3] {
        // SAFETY: integers are plain old datatypes so we can always transmute them to
        // arrays of bytes
        let mem: [u8; 4] = unsafe { mem::transmute(self) };
        mem[1..].try_into().unwrap()
    }
}
