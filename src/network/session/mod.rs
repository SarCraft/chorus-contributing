pub mod state;

use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use bedrockrs::network::connection::Connection;
use bedrockrs::network::connection::shard::arc::{shard, ConnectionShared};
use bedrockrs::proto::{Packets, Unknown, V944};
use bedrockrs::proto::v662::enums::{ConnectionFailReason, PlayStatus};
use bedrockrs::proto::v662::packets::PlayStatusPacket;
use bedrockrs::proto::v712::packets::{DisconnectMessage, DisconnectPacket};
use log::{info};
use statig::awaitable::{IntoStateMachineExt, StateMachine};
use crate::network::handler::packet_handler::PacketHandler;
use crate::network::session::state::SessionStateMachine;

pub struct Session {
    connection_shard: ConnectionShared<V944>,
    pub packet_handler: PacketHandler,
    closed: AtomicBool,
    state: StateMachine<SessionStateMachine>
}

impl Session {
    pub fn new(conn: Connection<Unknown>) -> Self {
        Self {
            connection_shard: shard(conn.into_ver()),
            packet_handler: PacketHandler::StartSession,
            closed: AtomicBool::new(false),
            state: SessionStateMachine::new().state_machine(),
        }
    }

    pub async fn tick(&mut self) -> Result<(), Box<dyn Error>> {
        self.connection_shard.send().await?;
        self.connection_shard.recv().await?;

        while let Some(packet) = self.connection_shard.read().await {
            info!("Packet: {:?}", packet.id());
            
            self.packet_handler.clone().handle(self, packet).await;
        }
        
        Ok(())
    }

    pub async fn on_login_success(&mut self) {
        self.send_play_status(PlayStatus::LoginSuccess, false).await;
    }

    pub async fn send_play_status(&mut self, status: PlayStatus, immediate: bool) {
        info!("Sending play status: {:?}", status);
        
        self.connection_shard.write(
            V944::PlayStatusPacket(
                PlayStatusPacket {
                    status
                }
            )
        ).await.unwrap();
        
        if (immediate) { self.connection_shard.send().await.unwrap() }
    }

    pub fn get_connection_shard(&self) -> &ConnectionShared<V944> {
        &self.connection_shard
    }

    pub fn get_mut_connection_shard(&mut self) -> &mut ConnectionShared<V944> {
        &mut self.connection_shard
    }
    
    pub fn get_state(&self) -> &StateMachine<SessionStateMachine> {
        &self.state
    }
    
    pub fn get_mut_state(&mut self) -> &mut StateMachine<SessionStateMachine> {
        &mut self.state
    }

    pub async fn close(&mut self, reason: Option<&str>) {
        if self.is_closed() { return; }

        if let Some(reason) = reason {
            self.connection_shard.write(V944::DisconnectPacket(
                DisconnectPacket {
                    reason: ConnectionFailReason::Disconnected,
                    message: Some(DisconnectMessage {
                        kick_message: reason.to_string(),
                        filtered_message: reason.to_string(),
                    })
                }
            )).await.unwrap();

            self.connection_shard.send().await.unwrap();
        }

        self.connection_shard.close().await;

        self.closed.store(true, Ordering::SeqCst)
    }
    
    pub fn is_closed(&self) -> bool { self.closed.load(Ordering::SeqCst) }
}