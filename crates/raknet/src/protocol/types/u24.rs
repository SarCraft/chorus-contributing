use std::io::{Error, Read, Write};
use crate::protocol::codec::RakCodec;

#[allow(non_camel_case_types)]
pub struct u24(pub [u8; 3]);

impl RakCodec for u24 {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_all(&self.0)
    }

    fn deserialize<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut buf = [0u8; 3];
        reader.read_exact(&mut buf)?;
        Ok(Self(buf))
    }

    fn size_hint(&self) -> usize {
        size_of::<Self>()
    }
}

impl From<u24> for u32 {
    fn from(u: u24) -> u32 {
       ((u.0[0] as u32) << 16) | ((u.0[1] as u32) << 8) | (u.0[2] as u32)
    }
}

impl From<u32> for u24 {
    fn from(u: u32) -> u24 {
        u24([
            (u >> 16) as u8,
            (u >> 8) as u8,
            u as u8,
        ])
    }
}