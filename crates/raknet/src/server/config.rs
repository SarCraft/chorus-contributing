use rand::random;
use crate::util::constants;

#[derive(Clone, Debug)]
pub struct RakServerConfig {
    pub max_ordering_channels: i32,
    pub guid: u64,
    pub protocols: Vec<u8>,
    pub max_connections: i32,
    pub magic: Vec<u8>,
    pub message: Vec<u8>,
    pub min_mtu_size: u16,
    pub max_mtu_size: u16,
    pub packet_limit: i32,
    pub total_packet_limit: i32,
    pub security: bool,
}

impl Default for RakServerConfig {
    fn default() -> RakServerConfig {
        Self {
            max_ordering_channels: constants::MAX_ORDERING_CHANNELS,
            guid: random(),
            protocols: vec![constants::PROTOCOL],
            max_connections: 10,
            magic: constants::MAGIC.to_vec(),
            message: vec![],
            min_mtu_size: constants::MIN_MTU_SIZE,
            max_mtu_size: constants::MAX_MTU_SIZE,
            packet_limit: constants::PACKET_LIMIT,
            total_packet_limit: constants::TOTAL_PACKET_LIMIT,
            security: false
        }
    }
}