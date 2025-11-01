use crate::protocol::codec::RakCodec;
use crate::protocol::types::frame::Frame;
use crate::util::flags::{CONTINUOUS_SEND, NEEDS_B_AND_AS, PAIR, VALID};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Error, ErrorKind, Read, Write};
use std::time::SystemTime;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FrameSet {
    sequence: u32,
    frames: Vec<Frame>,
    continuous_send: bool,
    needs_b_and_as: bool,
    is_pair: bool,
    
    pub sent: SystemTime,
    pub resend: SystemTime,
}

impl FrameSet {
    pub fn new(sequence: u32, frames: Vec<Frame>, continuous_send: bool, needs_b_and_as: bool, is_pair: bool) -> Self {
        Self { 
            sequence, 
            frames, 
            continuous_send, 
            needs_b_and_as, 
            is_pair,
            sent: SystemTime::now(),
            resend: SystemTime::UNIX_EPOCH,
        }
    }
}

impl RakCodec for FrameSet {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        let mut flags = VALID;
        if self.continuous_send { flags |= CONTINUOUS_SEND; }
        if self.needs_b_and_as { flags |= NEEDS_B_AND_AS; }
        if self.is_pair { flags |= PAIR; }
        
        writer.write_u8(flags)?;
        writer.write_u24::<LittleEndian>(self.sequence)?;
        for frame in &self.frames {
            frame.serialize(writer)?;
        }
        
        Ok(())
    }

    fn deserialize<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let flags = reader.read_u8()?;
        if flags & VALID != VALID {
            return Err(Error::new(ErrorKind::InvalidData, "not a FrameSet"));
        }
        
        let sequence = reader.read_u24::<LittleEndian>()?;
        
        let mut frames = Vec::new();
        loop {
            match Frame::deserialize(reader) {
                Ok(frame) => frames.push(frame),
                Err(e) => {
                    if e.kind() == ErrorKind::UnexpectedEof { break }
                    return Err(e);
                }
            }
        }
        
        let continuous_send = flags & CONTINUOUS_SEND != 0;
        let needs_b_and_as = flags & NEEDS_B_AND_AS != 0;
        let is_pair = flags & PAIR != 0;
        
        Ok(Self::new(sequence, frames, continuous_send, needs_b_and_as, is_pair))
    }

    fn size_hint(&self) -> usize {
        size_of::<u8>() + 3 + self.frames.iter().fold(0, |acc, frame| acc + frame.size_hint())
    }
}