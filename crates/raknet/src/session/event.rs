use std::net::SocketAddr;

pub enum RakSessionEvent {
    Connected(SocketAddr),
    Inbound(Vec<u8>, SocketAddr),
    Outbound(Vec<u8>, SocketAddr),
    Disconnect(SocketAddr),
}
