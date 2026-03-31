pub mod state;

use crate::network::session::state::SessionState;
use bedrockrs::network::connection::Connection;
use bedrockrs::proto::compression::Compression;
use bedrockrs::proto::v662::enums::{ConnectionFailReason, PlayStatus};
use bedrockrs::proto::v662::packets::PlayStatusPacket;
use bedrockrs::proto::v712::packets::{DisconnectMessage, DisconnectPacket};
use bedrockrs::proto::{Unknown, V944};
use bevy_ecs::prelude::Component;
use std::mem::take;
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::task::JoinHandle;
use tracing::{debug, error};

pub enum ConnectionEvent {
    SetCompression(Option<Compression>),
}

#[derive(Component)]
pub struct Session {
    closed: bool,

    pub state: SessionState,

    out_q: Vec<V944>,
    out_tx: UnboundedSender<Vec<V944>>,
    inc_rx: UnboundedReceiver<V944>,

    conn_tx: UnboundedSender<ConnectionEvent>,
    conn_task: JoinHandle<()>,
}

impl Session {
    pub fn new(conn: Connection<Unknown>, runtime: &tokio::runtime::Runtime) -> Self {
        let (out_tx, mut out_rx) = tokio::sync::mpsc::unbounded_channel::<Vec<V944>>();
        let (inc_tx, inc_rx) = tokio::sync::mpsc::unbounded_channel::<V944>();

        let (conn_tx, mut conn_rx) = tokio::sync::mpsc::unbounded_channel::<ConnectionEvent>();

        let mut conn: Connection<V944> = conn.into_ver();

        let conn_task = runtime.spawn(async move {
            loop {
                if conn.is_closed().await {
                    break;
                }

                while let Ok(event) = conn_rx.try_recv() {
                    match event {
                        ConnectionEvent::SetCompression(compression) => {
                            debug!("Setting compression to {:?}", compression);

                            conn.compression = compression;
                        }
                    }
                }

                tokio::select! {
                    recv = conn.recv() => {
                        match recv {
                            Ok(packets) => {
                                for packet in packets {
                                    if inc_tx.send(packet).is_err() { break; }
                                }
                            },
                            Err(err) => {
                                error!("error receiving packets from connection {:?}", err);
                                break;
                            }
                        }
                    }
                    Some(packets) = out_rx.recv() => {
                        if (!packets.is_empty()) {
                            debug!("Sending packets: {:?}", packets);

                            if let Err(err) = conn.send(&packets).await {
                                error!("error sending packets to connection {:?}", err);
                                break;
                            }
                        }
                    },
                }
            }
            conn.close().await;
        });

        Self {
            closed: false,

            state: SessionState::Start,

            out_q: vec![],
            out_tx,
            inc_rx,

            conn_tx,
            conn_task,
        }
    }

    pub fn send_immediate(&self, packet: V944) {
        _ = self.out_tx.send(vec![packet]);
    }

    pub fn send(&mut self, packet: V944) {
        self.out_q.push(packet);
    }

    pub fn flush(&mut self) {
        let out = take(&mut self.out_q);
        if (!out.is_empty()) {
            _ = self.out_tx.send(out);
        }
    }

    pub fn recv(&mut self) -> Option<V944> {
        match self.inc_rx.try_recv() {
            Ok(packet) => Some(packet),
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => {
                self.close(None);
                None
            }
        }
    }

    pub fn set_compression(&self, compression: Option<Compression>) {
        _ = self
            .conn_tx
            .send(ConnectionEvent::SetCompression(compression));
    }

    pub fn close(&mut self, reason: Option<&str>) {
        if self.is_closed() {
            return;
        }

        if let Some(reason) = reason {
            self.send_immediate(V944::DisconnectPacket(DisconnectPacket {
                reason: ConnectionFailReason::Disconnected,
                message: Some(DisconnectMessage {
                    kick_message: reason.to_string(),
                    filtered_message: reason.to_string(),
                }),
            }));
        }

        self.closed = true;
    }

    pub fn is_closed(&self) -> bool {
        self.closed
    }

    pub fn on_login_success(&mut self) {
        self.send_play_status(PlayStatus::LoginSuccess, false);
    }

    pub fn send_play_status(&mut self, status: PlayStatus, immediate: bool) {
        debug!("Sending play status: {:?}", status);

        if (immediate) {
            _ = self.send_immediate(V944::PlayStatusPacket(PlayStatusPacket { status }));
        } else {
            _ = self.send(V944::PlayStatusPacket(PlayStatusPacket { status }))
        }
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        self.conn_task.abort();
    }
}
