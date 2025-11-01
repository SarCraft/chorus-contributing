use std::io::{Error, ErrorKind, Read, Write};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use crate::protocol::codec::RakCodec;
use crate::util::packet_id::CONNECTION_REQUEST;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConnectionRequest {
    client_guid: u64,
    client_timestamp: u64,
    security: bool
}

impl ConnectionRequest {
    pub fn new(client_guid: u64, client_timestamp: u64, security: bool) -> Self {
        Self { client_guid, client_timestamp, security }
    }
    
    pub fn get_client_guid(&self) -> u64 {
        self.client_guid
    }
    
    pub fn get_client_timestamp(&self) -> u64 {
        self.client_timestamp
    }
    
    pub fn get_security(&self) -> bool {
        self.security
    }
}

impl RakCodec for ConnectionRequest {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_u8(CONNECTION_REQUEST)?;
        writer.write_u64::<BigEndian>(self.client_guid)?;
        writer.write_u64::<BigEndian>(self.client_timestamp)?;
        writer.write_u8(self.security as u8)?;
        
        Ok(())
    }

    fn deserialize<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let id = reader.read_u8()?;
        if id != CONNECTION_REQUEST {
            return Err(Error::new(ErrorKind::InvalidData, "not a ConnectionRequest"));
        }
        
        let client_guid = reader.read_u64::<BigEndian>()?;
        let client_timestamp = reader.read_u64::<BigEndian>()?;
        let security = reader.read_u8()? != 0;
        
        Ok(Self { client_guid, client_timestamp, security })
    }

    fn size_hint(&self) -> usize {
        size_of::<u8>() + size_of::<u64>() + size_of::<u64>() + size_of::<u8>()
    }
}