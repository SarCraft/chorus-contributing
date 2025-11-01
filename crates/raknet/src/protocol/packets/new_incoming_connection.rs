use std::io::{Cursor, Error, ErrorKind, Read, Write};
use std::net::SocketAddr;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use crate::protocol::codec::RakCodec;
use crate::util::packet_id::NEW_INCOMING_CONNECTION;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewIncomingConnection {
    server_address: SocketAddr,
    internal_addresses: Vec<SocketAddr>,
    incoming_timestamp: u64,
    server_timestamp: u64,
}

impl NewIncomingConnection {
    pub fn new(server_address: SocketAddr, internal_addresses: Vec<SocketAddr>, incoming_timestamp: u64, server_timestamp: u64) -> Self {
        Self { server_address, internal_addresses, incoming_timestamp, server_timestamp }
    }
    
    pub fn get_server_address(&self) -> &SocketAddr {
        &self.server_address
    }
    
    pub fn get_internal_addresses(&self) -> &Vec<SocketAddr> {
        &self.internal_addresses
    }
    
    pub fn get_incoming_timestamp(&self) -> u64 {
        self.incoming_timestamp
    }
    
    pub fn get_server_timestamp(&self) -> u64 {
        self.server_timestamp
    }
}

impl RakCodec for NewIncomingConnection {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_u8(NEW_INCOMING_CONNECTION)?;
        self.server_address.serialize(writer)?;
        for addr in &self.internal_addresses {
            addr.serialize(writer)?;
        }
        writer.write_u64::<BigEndian>(self.incoming_timestamp)?;
        writer.write_u64::<BigEndian>(self.server_timestamp)?;
        
        Ok(())
    }

    fn deserialize<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let id = reader.read_u8()?;
        if id != NEW_INCOMING_CONNECTION {
            return Err(Error::new(ErrorKind::InvalidData, "not a NewIncomingConnection"));
        }
        
        let server_address = SocketAddr::deserialize(reader)?;
        
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        
        let mut reader = Cursor::new(buf);
        
        let mut internal_addresses = Vec::new();
        while reader.get_ref().len() - reader.position() as usize > 16 {
            internal_addresses.push(SocketAddr::deserialize(&mut reader)?);
        }
        let incoming_timestamp = reader.read_u64::<BigEndian>()?;
        let server_timestamp = reader.read_u64::<BigEndian>()?;
        
        Ok(Self { server_address, internal_addresses, incoming_timestamp, server_timestamp })
    }

    fn size_hint(&self) -> usize {
        size_of::<u8>() + self.server_address.size_hint() + self.internal_addresses.iter().fold(0, |acc, addr| { 
            acc + addr.size_hint() 
        }) + size_of::<u64>() + size_of::<u64>()
    }
}