use crate::protocol::codec::RakCodec;
use crate::util::constants::MAGIC;
use crate::util::packet_id::OPEN_CONNECTION_REPLY_1;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Error, ErrorKind, Read, Write};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpenConnectionReply1 {
    guid: u64,
    cookie: Option<i32>,
    mtu: u16,
}

impl OpenConnectionReply1 {
    pub fn new(guid: u64, cookie: Option<i32>, mtu: u16) -> Self {
        Self { guid, cookie, mtu }
    }

    pub fn get_guid(&self) -> u64 {
        self.guid
    }

    pub fn get_cookie(&self) -> Option<i32> {
        self.cookie
    }

    pub fn get_mtu(&self) -> u16 {
        self.mtu
    }
}

impl RakCodec for OpenConnectionReply1 {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_u8(OPEN_CONNECTION_REPLY_1)?;
        writer.write_all(&MAGIC)?;
        writer.write_u64::<BigEndian>(self.guid)?;
        match self.cookie {
            Some(cookie) => {
                writer.write_u8(1)?;
                writer.write_i32::<BigEndian>(cookie)?;
            }
            None => writer.write_u8(0)?,
        }
        writer.write_u16::<BigEndian>(self.mtu)?;

        Ok(())
    }

    fn deserialize<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let id = reader.read_u8()?;
        if id != OPEN_CONNECTION_REPLY_1 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "not an OpenConnectionReply1",
            ));
        }

        let mut magic = [0u8; MAGIC.len()];
        reader.read_exact(&mut magic)?;

        if magic != MAGIC {
            return Err(Error::new(ErrorKind::InvalidData, "invalid magic"));
        }

        let guid = reader.read_u64::<BigEndian>()?;
        let cookie = if reader.read_u8()? != 0 {
            Some(reader.read_i32::<BigEndian>()?)
        } else {
            None
        };
        let mtu = reader.read_u16::<BigEndian>()?;

        Ok(Self { guid, cookie, mtu })
    }

    fn size_hint(&self) -> usize {
        size_of::<u8>()
            + MAGIC.len()
            + size_of::<u64>()
            + size_of::<u8>()
            + if matches!(&self.cookie, Some(_)) {
                size_of::<i32>()
            } else {
                0
            }
            + size_of::<u16>()
    }
}
