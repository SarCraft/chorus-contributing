use std::io::{Error, ErrorKind, Read, Write};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use crate::protocol::codec::RakCodec;
use crate::util::{constants, packet_id};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnconnectedPing {
    timestamp: u64,
    client: u64,
}

impl UnconnectedPing {
    pub fn new(timestamp: u64, client: u64) -> Self {
        Self { timestamp, client }
    }
    
    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }
    
    pub fn get_client(&self) -> u64 {
        self.client
    }
}

impl RakCodec for UnconnectedPing {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_u8(packet_id::UNCONNECTED_PING)?;
        writer.write_u64::<BigEndian>(self.timestamp)?;
        writer.write_all(&constants::MAGIC)?;
        writer.write_u64::<BigEndian>(self.client)?;
        
        Ok(())
    }

    fn deserialize<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let id = reader.read_u8()?;
        if id != packet_id::UNCONNECTED_PING {
            return Err(Error::new(ErrorKind::InvalidData, "not an UnconnectedPing"));
        }
        
        let timestamp = reader.read_u64::<BigEndian>()?;
        let mut magic = [0u8; constants::MAGIC.len()];
        reader.read_exact(&mut magic)?;
        
        if magic != constants::MAGIC {
            return Err(Error::new(ErrorKind::InvalidData, "invalid magic"));
        }
        
        let client = reader.read_u64::<BigEndian>()?;
        
        Ok(Self { timestamp, client})
    }

    fn size_hint(&self) -> usize {
        size_of::<u8>() + size_of::<u64>() + constants::MAGIC.len() + size_of::<u64>()
    }
}