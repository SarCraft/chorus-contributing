use std::io::{Error, ErrorKind, Read, Write};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use crate::protocol::codec::RakCodec;
use crate::util::{constants, packet_id};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnconnectedPong {
    timestamp: u64,
    guid: u64,
    message: Vec<u8>,
}

impl UnconnectedPong {
    pub fn new(timestamp: u64, guid: u64, message: Vec<u8>) -> Self {
        Self { timestamp, guid, message }
    }
    
    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }
    
    pub fn get_guid(&self) -> u64 {
        self.guid
    }
    
    pub fn get_message(&self) -> &Vec<u8> {
        &self.message
    }
}

impl RakCodec for UnconnectedPong {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_u8(packet_id::UNCONNECTED_PONG)?;
        writer.write_u64::<BigEndian>(self.timestamp)?;
        writer.write_u64::<BigEndian>(self.guid)?;
        writer.write_all(&constants::MAGIC)?;
        writer.write_u16::<BigEndian>(self.message.len() as u16)?;
        writer.write_all(&self.message)?;
        
        Ok(())
    }

    fn deserialize<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let id = reader.read_u8()?;
        if id != packet_id::UNCONNECTED_PONG {
            return Err(Error::new(ErrorKind::InvalidData, "not an UnconnectedPong"));
        }
        
        let timestamp = reader.read_u64::<BigEndian>()?;
        let guid = reader.read_u64::<BigEndian>()?;
        
        let mut magic = [0u8; constants::MAGIC.len()];
        reader.read_exact(&mut magic)?;
        if magic != constants::MAGIC {
            return Err(Error::new(ErrorKind::InvalidData, "invalid magic"));
        }
        
        let message_len = reader.read_u16::<BigEndian>()?;
        let mut message = vec![0u8; message_len as usize];
        reader.read_exact(&mut message)?;
        
        Ok(Self { timestamp, guid, message })
    }

    fn size_hint(&self) -> usize {
        size_of::<u8>() + size_of::<u64>() + size_of::<u64>() + constants::MAGIC.len() + size_of::<u16>() + self.message.len()
    }
}