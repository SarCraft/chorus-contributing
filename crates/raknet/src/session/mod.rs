pub mod config;
pub mod event;
pub mod state;

use std::io::Cursor;
use std::net::SocketAddr;
use std::ops::{Add, Deref};
use std::sync::{Arc};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tracing::debug;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use crate::protocol::codec::RakCodec;
use crate::protocol::packets::connected_ping::ConnectedPing;
use crate::protocol::packets::connection_request::ConnectionRequest;
use crate::protocol::packets::connection_request_accepted::ConnectionRequestAccepted;
use crate::protocol::packets::new_incoming_connection::NewIncomingConnection;
use crate::protocol::types::frame::Frame;
use crate::session::config::RakSessionConfig;
use crate::session::event::RakSessionEvent;
use crate::session::state::RakSessionState;
use crate::types::priority::RakPriority;
use crate::types::reliability::RakReliability;
use crate::util::constants::UDP_HEADER_SIZE;
use crate::util::socket_addr::get_overhead;

pub struct RakSession {
    event_tx: UnboundedSender<RakSessionEvent>,
    
    addr: SocketAddr,
    guid: u64,
    mtu: u16,
    config: RakSessionConfig,
    
    state: RakSessionState,
    
    last_update: Arc<RwLock<SystemTime>>,
    
    curr_ping: Arc<RwLock<SystemTime>>,
    last_ping: Arc<RwLock<SystemTime>>,
    last_pong: Arc<RwLock<SystemTime>>,
    
    update_task: Option<JoinHandle<()>>,
    
    in_tx: UnboundedSender<Vec<u8>>,
    out_tx: UnboundedSender<(Frame, RakPriority)>,
}

impl RakSession {
    pub fn new<F>(event_tx: UnboundedSender<RakSessionEvent>, addr: SocketAddr, guid: u64, mtu: u16, conf: F) -> Self
    where 
        F: FnOnce(&mut RakSessionConfig)
    {
        let (in_tx, in_rx) = unbounded_channel();
        let (out_tx, out_rx) = unbounded_channel();
        let mtu = mtu - UDP_HEADER_SIZE - get_overhead(&addr);
        let mut config = RakSessionConfig::default();
        conf(&mut config);
        let mut s = Self {
            event_tx,
            addr,
            guid,
            mtu,
            config,
            
            state: RakSessionState::Connecting,
            
            last_update: Arc::new(RwLock::new(SystemTime::now())),
            
            curr_ping: Arc::new(RwLock::new(SystemTime::UNIX_EPOCH)),
            last_ping: Arc::new(RwLock::new(SystemTime::UNIX_EPOCH)),
            last_pong: Arc::new(RwLock::new(SystemTime::UNIX_EPOCH)),
            
            update_task: None,
            
            in_tx,
            out_tx,
        };
        
        s.update_task = Some(s.setup_update_task(in_rx, out_rx));
        s
    }
    
    pub fn get_addr(&self) -> SocketAddr {
        self.addr
    }
    
    pub fn send(&self, buf: Vec<u8>, reliability: RakReliability, priority: RakPriority) {
        _ = self.out_tx.send((Frame::new(reliability, buf), priority));
    }
    
    pub fn inbound(&self, buf: Vec<u8>) {
        _ = self.in_tx.send(buf);
    }

    pub fn handle_connection_request(&self, buf: &mut Cursor<&[u8]>) {
        match self.state {
            RakSessionState::Connecting => (),
            _ => return debug!("unexpected ConnectionRequest from {}", self.addr)
        }

        let Ok(request) = ConnectionRequest::deserialize(buf) else {
            return debug!("failed to deserialize ConnectionRequest from {}", self.addr);
        };

        let accepted = ConnectionRequestAccepted::new(
            self.addr,
            0,
            vec![],
            request.get_client_timestamp(),
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64,
        );

        let mut buf = Vec::with_capacity(accepted.size_hint());
        accepted.serialize(&mut buf).unwrap();

        self.send(buf, RakReliability::ReliableOrdered, RakPriority::Normal);
    }

    pub fn handle_new_incoming_connection(&mut self, buf: &mut Cursor<&[u8]>) {
        match self.state {
            RakSessionState::Connecting => (),
            _ => return debug!("unexpected NewIncomingConnection from {}", self.addr)
        }

        let Ok(_) = NewIncomingConnection::deserialize(buf) else {
            return debug!("failed to deserialize NewIncomingConnection from {}", self.addr);
        };

        self.state = RakSessionState::Connected;
        _ = self.event_tx.send(RakSessionEvent::Connected(self.addr));
    }
    
    fn setup_update_task(&self, in_rx: UnboundedReceiver<Vec<u8>>, out_rx: UnboundedReceiver<(Frame, RakPriority)>) -> JoinHandle<()> {
        tokio::spawn({
            let mut in_rx = in_rx;
            let mut out_rx = out_rx;
            
            let last_update = self.last_update.clone();
            let curr_ping = self.curr_ping.clone();
            async move {
                let mut last_tick = SystemTime::now();
                loop {
                    while let Ok((_, _)) = out_rx.try_recv() {}
                    while let Ok(_) = in_rx.try_recv() {}
                    
                    let now = SystemTime::now();
                    if last_tick + Duration::from_millis(10) <= now {
                        // tick()
                        last_tick = now;
                    }
                    
                    if curr_ping.read().await.add(Duration::from_millis(2000)) <= now {
                        let ping = ConnectedPing::new(now.duration_since(UNIX_EPOCH).unwrap().as_millis() as u64);
                        
                        let mut buf = Vec::with_capacity(ping.size_hint());
                        ping.serialize(&mut buf).unwrap();
                    }
                }
            } 
        })
    }
}