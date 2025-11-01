use std::io::{Cursor, Error, ErrorKind, Read, Write};
use std::net::SocketAddr;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use crate::protocol::codec::RakCodec;
use crate::util::packet_id::CONNECTION_REQUEST_ACCEPTED;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConnectionRequestAccepted {
    client_address: SocketAddr,
    system_index: u16,
    system_addresses: Vec<SocketAddr>,
    request_timestamp: u64,
    timestamp: u64,
}

impl ConnectionRequestAccepted {
    pub fn new(client_address: SocketAddr, system_index: u16, system_addresses: Vec<SocketAddr>, request_timestamp: u64, timestamp: u64) -> Self {
        Self { client_address, system_index, system_addresses, request_timestamp, timestamp }
    }
    
    pub fn get_client_address(&self) -> SocketAddr {
        self.client_address
    }
    
    pub fn get_system_index(&self) -> u16 {
        self.system_index
    }
    
    pub fn get_system_addresses(&self) -> &Vec<SocketAddr> {
        &self.system_addresses
    }
    
    pub fn get_request_timestamp(&self) -> u64 {
        self.request_timestamp
    }
    
    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }
}

impl RakCodec for ConnectionRequestAccepted {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_u8(CONNECTION_REQUEST_ACCEPTED)?;
        self.client_address.serialize(writer)?;
        writer.write_u16::<BigEndian>(self.system_index)?;
        for addr in &self.system_addresses {
            addr.serialize(writer)?;
        }
        writer.write_u64::<BigEndian>(self.request_timestamp)?;
        writer.write_u64::<BigEndian>(self.timestamp)?;
        
        Ok(())
    }

    fn deserialize<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let id = reader.read_u8()?;
        if id != CONNECTION_REQUEST_ACCEPTED {
            return Err(Error::new(ErrorKind::InvalidData, "not a ConnectionRequestAccepted"));
        }
        
        let client_address = SocketAddr::deserialize(reader)?;
        let system_index = reader.read_u16::<BigEndian>()?;
        
        let mut system_addresses = Vec::new();
        
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        
        let mut reader = Cursor::new(buf);
        while reader.get_ref().len() - reader.position() as usize > 16 {
            system_addresses.push(SocketAddr::deserialize(&mut reader)?);
        }
        
        let request_timestamp = reader.read_u64::<BigEndian>()?;
        let timestamp = reader.read_u64::<BigEndian>()?;
        
        Ok(Self { client_address, system_index, system_addresses, request_timestamp, timestamp })
    }

    fn size_hint(&self) -> usize {
        size_of::<u8>() + self.client_address.size_hint() + size_of::<u16>() + self.system_addresses.iter().fold(0, |acc, addr| {
            acc + addr.size_hint()
        }) + size_of::<u64>() + size_of::<u64>()
    }
}