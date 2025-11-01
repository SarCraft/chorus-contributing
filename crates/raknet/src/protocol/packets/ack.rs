use std::io::{Error, ErrorKind, Read, Write};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use crate::protocol::codec::RakCodec;
use crate::protocol::types::u24::u24;
use crate::util::flags;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ack {
    is_nack: bool,
    sequences: Vec<u32>
}

impl Ack {
    pub fn new(sequences: Vec<u32>, is_nack: bool) -> Self {
        let mut sorted = sequences.clone();
        sorted.sort_unstable();
        sorted.dedup();
        Self { is_nack, sequences: sorted }
    }
    
    pub fn get_sequences(&self) -> &Vec<u32> {
        &self.sequences
    }
    
    pub fn is_nack(&self) -> bool {
        self.is_nack
    }
    
    #[inline(always)]
    fn serialize_range<W: Write>(start: u32, end: u32, writer: &mut W) -> Result<(), Error> {
        if start == end {
            writer.write_u8(1)?;
            u24::from(start).serialize(writer)?;
        } else {
            writer.write_u8(0)?;
            u24::from(start).serialize(writer)?;
            u24::from(end).serialize(writer)?;
        }
        Ok(())
    }

    #[inline(always)]
    fn range_size_hint(start: u32, end: u32) -> usize {
        size_of::<u8>() + size_of::<u24>() + if start == end { 0 } else { size_of::<u24>() }
    }
}

impl RakCodec for Ack {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_u8(
            flags::VALID | if self.is_nack { flags::NACK } else { flags::ACK }
        )?;
        
        let (&first, rest) = match self.sequences.split_first() {
            Some(pair) => pair,
            None => {
                writer.write_u16::<BigEndian>(0)?;
                return Ok(()) 
            },
        };
        
        // in worst case each sequence is written as a 4 byte single-value range
        let mut buf: Vec<u8> = Vec::with_capacity(self.sequences.len() * 4);
        let mut count: u16 = 0;
        
        let mut start: u32 = first;
        let mut end: u32 = start;
        for &i in rest {
            if i == end + 1 {
                end = i
            } else {
                Self::serialize_range(start, end, &mut buf)?;
                count += 1;
                start = i;
                end = i;
            }
        }
        Self::serialize_range(start, end, &mut buf)?;
        count += 1;
        
        writer.write_u16::<BigEndian>(count)?;
        writer.write_all(&buf)?;

        Ok(())
    }

    fn deserialize<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let id = reader.read_u8()?;
        if id & flags::VALID == 0 || (id & (flags::ACK | flags::NACK)).count_ones() != 1 {
            return Err(Error::new(ErrorKind::InvalidInput, "invalid, not an ack or nack"));
        }
        
        let is_nack = id & flags::NACK != 0;
        
        let count = reader.read_u16::<BigEndian>()?;
        
        let mut sequences: Vec<u32> = Vec::new();
        for _ in 0..count {
            if reader.read_u8()? != 0 {
                sequences.push(u24::deserialize(reader)?.into());
            } else {
                let start: u32 = u24::deserialize(reader)?.into();
                let end: u32 = u24::deserialize(reader)?.into();
                if end < start {
                    return Err(Error::new(ErrorKind::InvalidData, "invalid range, end < start"));
                }
                sequences.extend(start..end);
            }
        }
        
        Ok(Self { is_nack, sequences })
    }

    fn size_hint(&self) -> usize {
        let mut size = size_of::<u8>() + size_of::<u16>();

        let (&first, rest) = match self.sequences.split_first() {
            Some(pair) => pair,
            None => { return size; },
        };
        
        let mut start: u32 = first;
        let mut end: u32 = start;
        for &i in rest {
            if i == end + 1 {
                end = i
            } else {
                size += Self::range_size_hint(start, end);
                start = i;
                end = i;
            }
        }
        size += Self::range_size_hint(start, end);
        
        size
    }
}