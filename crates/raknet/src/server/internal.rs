use std::collections::HashMap;
use std::io::Cursor;
use std::net::SocketAddr;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use log::debug;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use tokio::sync::RwLock;
use crate::protocol::codec::RakCodec;
use crate::protocol::packets::incompatible_protocol::IncompatibleProtocol;
use crate::protocol::packets::open_connection_reply_1::OpenConnectionReply1;
use crate::protocol::packets::open_connection_reply_2::OpenConnectionReply2;
use crate::protocol::packets::open_connection_request_1::OpenConnectionRequest1;
use crate::protocol::packets::open_connection_request_2::OpenConnectionRequest2;
use crate::protocol::packets::unconnected_ping::UnconnectedPing;
use crate::protocol::packets::unconnected_pong::UnconnectedPong;
use crate::server::config::RakServerConfig;
use crate::session::event::RakSessionEvent;
use crate::session::RakSession;
use crate::util::constants::{PROTOCOL, UDP_HEADER_SIZE};
use crate::util::flags::VALID;
use crate::util::packet_id;
use crate::util::socket_addr::get_overhead;

#[derive(Clone)]
pub struct RakServerInternal {
    config: Arc<RwLock<RakServerConfig>>,
    addr: SocketAddr,
    
    sessions: Arc<RwLock<HashMap<SocketAddr, RwLock<RakSession>>>>,
    
    pub out_tx: UnboundedSender<(Vec<u8>, SocketAddr)>, 
}

impl RakServerInternal {
    pub fn new(config: Arc<RwLock<RakServerConfig>>, addr: SocketAddr, out_tx: UnboundedSender<(Vec<u8>, SocketAddr)>) -> Self {
        Self { 
            config, 
            addr,
            sessions: Arc::new(RwLock::new(HashMap::new())),
            out_tx,
        }
    }
    
    pub async fn handle(&mut self, buf: &[u8], addr: SocketAddr) {
        if let Some(&header) = buf.first() {
            match header & VALID {
                0 => self.handle_offline(buf, addr).await,
                _ => if let Some(s) = self.sessions.read().await.get(&addr) {
                    _ = s.read().await.inbound(buf.to_vec());
                }
            }
        }
    }
    
    async fn handle_offline(&mut self, buf: &[u8], addr: SocketAddr) {
        if let Some(&id) = buf.first() {
            let mut cursor = Cursor::new(buf);
            match id {
                packet_id::UNCONNECTED_PING => self.handle_unconnected_ping(&mut cursor, addr).await,
                packet_id::OPEN_CONNECTION_REQUEST_1 => self.handle_open_connection_request_1(&mut cursor, addr).await,
                packet_id::OPEN_CONNECTION_REQUEST_2 => self.handle_open_connection_request_2(&mut cursor, addr).await,

                _ => debug!("received unknown offline packet from {}, id: {:#04X}", addr, id)
            }
        }
    }

    async fn handle_unconnected_ping(&self, cursor: &mut Cursor<&[u8]>, addr: SocketAddr) {
        let Ok(ping) = UnconnectedPing::deserialize(cursor) else {
            return debug!("failed to deserialize UnconnectedPing from {}", addr);
        };
        
        let config = self.config.read().await;
        
        let pong = UnconnectedPong::new(
            ping.get_timestamp(),
            config.guid,
            config.message.clone()
        );

        let mut buf = Vec::with_capacity(pong.size_hint());
        pong.serialize(&mut buf).unwrap();
        
        self.send((buf, addr));
        
        debug!("ponged {} with {:?}", addr, pong)
    }
    
    async fn handle_open_connection_request_1(&self, cursor: &mut Cursor<&[u8]>, addr: SocketAddr) {
        let Ok(request) = OpenConnectionRequest1::deserialize(cursor) else {
            return debug!("failed to deserialize OpenConnectionRequest1 from {}", addr);
        };
        
        let config = self.config.read().await;

        let req_protocol = request.get_protocol();
        if req_protocol != PROTOCOL {
            let incompatible = IncompatibleProtocol::new(
                PROTOCOL,
                config.guid,
            );
            
            debug!("refusing connection from {} due to incompatible protocol {}, expected {}", addr, req_protocol, PROTOCOL);
            
            let mut buf = Vec::with_capacity(incompatible.size_hint());
            incompatible.serialize(&mut buf).unwrap();
            
            self.send((buf, addr));
            
            return;
        }
        
        let reply = OpenConnectionReply1::new(
            config.guid,
            None,
            (request.get_mtu() + UDP_HEADER_SIZE + get_overhead(&addr)).clamp(config.min_mtu_size, config.max_mtu_size)
        );
        
        let mut buf = Vec::with_capacity(reply.size_hint());
        reply.serialize(&mut buf).unwrap();
        
        self.send((buf, addr));
    }
    
    async fn handle_open_connection_request_2(&mut self, cursor: &mut Cursor<&[u8]>, addr: SocketAddr) {
        let Ok(request) = OpenConnectionRequest2::deserialize(cursor) else {
            return debug!("failed to deserialize OpenConnectionRequest2 from {}", addr);
        };
        
        let config = self.config.read().await;
        
        if request.get_address() != self.addr {
            return debug!("refusing connection from {} due to address mismatch", addr);
        }
        
        let mtu = request.get_mtu();
        
        if !(config.min_mtu_size..=config.max_mtu_size).contains(&mtu) {
            return debug!("refusing connection from {} due to invalid mtu size", addr)
        }
        
        if self.sessions.read().await.contains_key(&addr) {
            return debug!("refusing connection from {} due to existing connection", addr);
        }

        debug!("establishing connection from {} with mtu size of {}", addr, mtu);
        
        let reply = OpenConnectionReply2::new(
            config.guid,
            addr,
            mtu,
            false,
        );

        let mut buf = Vec::with_capacity(reply.size_hint());
        reply.serialize(&mut buf).unwrap();

        self.send((buf, addr));
        
        let (event_tx, mut event_rx) = unbounded_channel();
        
        self.sessions.write().await.insert(
            addr, 
            RwLock::new(
                RakSession::new(
                    event_tx,
                    addr,
                    request.get_client(),
                    request.get_mtu(),
                    |_| ()
                ) 
            )
        );
        
        tokio::spawn({
            let sessions = self.sessions.clone();
            async move {
                while let Some(event) = event_rx.recv().await {
                    match event {
                        RakSessionEvent::Connected(_) => {}
                        RakSessionEvent::Inbound(buf, addr) => {
                            if let Some(&b) = buf.first() && let Some(session) = sessions.read().await.get(&addr) {
                                let mut cursor = Cursor::new(buf.as_slice());
                                match b {
                                    packet_id::CONNECTION_REQUEST => session.read().await.deref().handle_connection_request(&mut cursor),
                                    packet_id::NEW_INCOMING_CONNECTION => session.write().await.deref_mut().handle_new_incoming_connection(&mut cursor),
                                    _ => debug!("packet from {}, id: {:#04X}", addr, b)
                                }
                            }
                        }
                        RakSessionEvent::Outbound(_, _) => {}
                        RakSessionEvent::Disconnect(addr) => {
                            sessions.write().await.remove(&addr);
                        }
                    }
                }
            }
        });
    }
    
    fn send(&self, packet: (Vec<u8>, SocketAddr)) {
        _ = self.out_tx.send(packet);
    }
}