use crate::protocol::codec::RakCodec;
use crate::util::constants::{MAGIC, MAX_MTU_SIZE};
use crate::util::packet_id::OPEN_CONNECTION_REQUEST_1;
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Error, ErrorKind, Read, Write};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpenConnectionRequest1 {
    protocol: u8,
    mtu: u16,
}

impl OpenConnectionRequest1 {
    pub fn new(protocol: u8, mtu: u16) -> Self {
        Self { protocol, mtu }
    }
    
    pub fn get_protocol(&self) -> u8 {
        self.protocol
    }
    
    pub fn get_mtu(&self) -> u16 {
        self.mtu
    }
}

impl RakCodec for OpenConnectionRequest1 {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        let mut buf: Vec<u8> = Vec::with_capacity(self.mtu as usize);
        
        buf.write_u8(OPEN_CONNECTION_REQUEST_1)?;
        buf.write_all(&MAGIC)?;
        buf.write_u8(self.protocol)?;
        buf.resize(self.mtu as usize, 0);
        
        writer.write_all(&buf)?;
        
        Ok(())
    }

    fn deserialize<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut buf: Vec<u8> = Vec::new();
        reader.read_to_end(&mut buf)?;
        
        let mtu = buf.len() as u16;
        
        let mut buf = Cursor::new(buf);
        
        let id = buf.read_u8()?;
        if id != OPEN_CONNECTION_REQUEST_1 {
            return Err(Error::new(ErrorKind::InvalidData, "not an OpenConnectionRequest1"));
        }
        
        let mut magic = [0u8; MAGIC.len()];
        buf.read_exact(&mut magic)?;
        
        if magic != MAGIC {
            return Err(Error::new(ErrorKind::InvalidData, "invalid magic"));
        }
        
        let protocol = buf.read_u8()?;
            
        Ok(Self { protocol, mtu })
    }

    fn size_hint(&self) -> usize {
        self.mtu as usize
    }
}