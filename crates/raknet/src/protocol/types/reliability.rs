#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Reliability {
    Unreliable,
    UnreliableSequenced,
    Reliable,
    ReliableOrdered,
    ReliableSequenced,
    UnreliableWithAckReceipt,
    ReliableWithAckReceipt,
    ReliableOrderedWithAckReceipt,
}

impl Reliability {
    pub fn is_reliable(&self) -> bool {
        match self { 
            Reliability::Reliable 
            | Reliability::ReliableOrdered
            | Reliability::ReliableWithAckReceipt
            | Reliability::ReliableOrderedWithAckReceipt => true,
            _ => false,
        }
    }
    
    pub fn is_sequenced(&self) -> bool {
        match self {
            Reliability::ReliableSequenced 
            | Reliability::UnreliableSequenced => true,
            _ => false,
        }
    }
    
    pub fn is_ordered(&self) -> bool {
        match self {
            Reliability::ReliableOrdered 
            | Reliability::ReliableOrderedWithAckReceipt => true,
            _ => false
        }
    }
}

impl TryFrom<u8> for Reliability {
    type Error = ();
    
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Reliability::Unreliable),
            1 => Ok(Reliability::UnreliableSequenced),
            2 => Ok(Reliability::Reliable),
            3 => Ok(Reliability::ReliableOrdered),
            4 => Ok(Reliability::ReliableSequenced),
            5 => Ok(Reliability::UnreliableWithAckReceipt),
            6 => Ok(Reliability::ReliableWithAckReceipt),
            7 => Ok(Reliability::ReliableOrderedWithAckReceipt),
            _ => Err(())
        }
    }
}