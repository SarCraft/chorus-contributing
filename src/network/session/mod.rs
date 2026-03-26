pub mod state;

use bedrockrs::network::connection::Connection;
use bedrockrs::proto::{Unknown, V944};
use bedrockrs::proto::compression::Compression;
use bedrockrs::proto::v662::enums::{PlayStatus};
use bedrockrs::proto::v662::packets::PlayStatusPacket;
use bevy_ecs::prelude::Component;
use crossbeam_channel::{Receiver, Sender};
use log::{error, info};
use tokio::task::JoinHandle;

pub enum ConnectionEvent {
    SetCompression(Option<Compression>)
}

#[derive(Component)]
pub struct Session {
    out_tx: Sender<V944>,
    inc_rx: Receiver<V944>,
    
    conn_tx: Sender<ConnectionEvent>,
    conn_task: JoinHandle<()>,
}

impl Session {
    pub fn new(conn: Connection<Unknown>, runtime: &tokio::runtime::Runtime) -> Self {
        let (out_tx, out_rx) = crossbeam_channel::unbounded::<V944>();
        let (inc_tx, inc_rx) = crossbeam_channel::unbounded::<V944>();
        
        let (conn_tx, conn_rx) = crossbeam_channel::unbounded::<ConnectionEvent>();
        
        let mut conn: Connection<V944> = conn.into_ver();
        
        let conn_task = runtime.spawn(async move {
            loop {
                for event in conn_rx.try_iter() {
                    match event { 
                        ConnectionEvent::SetCompression(compression) => {
                            conn.compression = compression;
                        }
                    }
                }
                
                match conn.recv().await {
                    Ok(packets) => {
                        for packet in packets {
                            if inc_tx.send(packet).is_err() { break; }
                        }
                    },
                    Err(err) => {
                        error!("error receiving packets from connection {:#?}", err);
                    }
                }

                for packet in out_rx.try_iter() {
                    if let Err(err) = conn.send(&[packet]).await {
                        error!("error sending packets to connection {:#?}", err);
                    }
                }
            };
        });
        
        Self {
            out_tx,
            inc_rx,
            
            conn_tx,
            conn_task,
        }
    }
    
    pub fn send(&self, packet: V944) -> anyhow::Result<()> {
        self.out_tx.try_send(packet)?;
        
        Ok(())
    }
    
    pub fn recv(&self) -> anyhow::Result<V944> {
        Ok(self.inc_rx.try_recv()?)
    }
    
    pub fn set_compression(&self, compression: Option<Compression>) -> anyhow::Result<()> {
        self.conn_tx.try_send(ConnectionEvent::SetCompression(compression))?;
        
        Ok(())
    }

    pub fn on_login_success(&mut self) {
        self.send_play_status(PlayStatus::LoginSuccess, false);
    }

    pub fn send_play_status(&self, status: PlayStatus, immediate: bool) {
        info!("Sending play status: {:?}", status);
        
        self.send(
            V944::PlayStatusPacket(
                PlayStatusPacket {
                    status
                }
            )
        ).unwrap();
        
        if (immediate) { todo!() }
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        self.conn_task.abort();
    }
}