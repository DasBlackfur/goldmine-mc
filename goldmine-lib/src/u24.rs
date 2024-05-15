use std::mem;

use declio::{ctx::Endian, Decode, Encode};
use serde::{Deserialize, Serialize};

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
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

    #[inline(always)]
    pub const fn from_le(x: Self) -> Self {
        #[cfg(target_endian = "little")]
        {
            x
        }
        #[cfg(not(target_endian = "little"))]
        {
            u24(x.0.swap_bytes())
        }
    }

    #[inline(always)]
    pub const fn to_le(self) -> Self {
        // or not to be?
        #[cfg(target_endian = "little")]
        {
            self
        }
        #[cfg(not(target_endian = "little"))]
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
    pub fn from_le_bytes(bytes: [u8; 3]) -> Self {
        Self::from_le(u24::from_ne_bytes(bytes))
    }

    #[inline]
    pub fn to_le_bytes(self) -> [u8; 3] {
        self.to_le().to_ne_bytes()
    }

    #[inline]
    pub fn from_ne_bytes(input_bytes: [u8; 3]) -> u24 {
        // SAFETY: integers are plain old datatypes so we can always transmute to them
        let mut bytes: [u8; 4] = [0; 4];
        #[cfg(target_endian = "big")]
        {
            bytes[1..].copy_from_slice(&input_bytes);
        }
        #[cfg(not(target_endian = "big"))]
        {
            bytes[..3].copy_from_slice(&input_bytes);
        }

        unsafe { mem::transmute(bytes) }
    }

    #[inline]
    pub fn to_ne_bytes(self) -> [u8; 3] {
        // SAFETY: integers are plain old datatypes so we can always transmute them to
        // arrays of bytes
        let mem: [u8; 4] = unsafe { mem::transmute(self) };
        #[cfg(target_endian = "big")]
        {
            mem[1..].try_into().unwrap()
        }
        #[cfg(not(target_endian = "big"))]
        {
            mem[..3].try_into().unwrap()
        }
    }
}

impl Encode<Endian> for u24 {
    fn encode<W>(&self, ctx: Endian, writer: &mut W) -> Result<(), declio::Error>
    where
        W: std::io::Write,
    {
        match ctx {
            Endian::Big => writer.write_all(&self.to_be_bytes())?,
            Endian::Little => writer.write_all(&self.to_le_bytes())?,
        }
        Ok(())
    }
}

impl Decode<Endian> for u24 {
    fn decode<R>(ctx: Endian, reader: &mut R) -> Result<Self, declio::Error>
    where
        R: std::io::Read,
    {
        let mut buf = [0_u8; 3];
        reader.read_exact(&mut buf)?;
        match ctx {
            Endian::Big => Ok(u24::from_be_bytes(buf)),
            Endian::Little => Ok(u24::from_le_bytes(buf)),
        }
    }
}

