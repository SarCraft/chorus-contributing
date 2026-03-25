pub mod state;

use bedrockrs::network::connection::Connection;
use bedrockrs::proto::{Packets, Unknown, V944};
use bedrockrs::proto::v662::enums::{PlayStatus};
use bedrockrs::proto::v662::packets::PlayStatusPacket;
use bevy_ecs::prelude::Component;
use bevy_ecs::system::Query;
use crossbeam_channel::{Receiver, Sender};
use log::{error, info};
use tokio::task::JoinHandle;

#[derive(Component)]
pub struct Session {
    outgoing: Sender<V944>,
    incoming: Receiver<V944>,
    conn_task: JoinHandle<()>,
}

impl Session {
    pub fn new(conn: Connection<Unknown>) -> Self {
        let (out_send, out_recv) = crossbeam_channel::unbounded::<V944>();
        let (in_send, in_recv) = crossbeam_channel::unbounded::<V944>();
        
        let mut conn: Connection<V944> = conn.into_ver();
        
        let conn_task = tokio::spawn(async move {
            loop {
                match conn.recv().await {
                    Ok(packets) => {
                        for packet in packets {
                            if in_send.send(packet).is_err() { break; }
                        }
                    },
                    Err(err) => {
                        error!("error receiving packets from connection {:#?}", err);
                    }
                }

                for packet in out_recv.try_iter() {
                    if let Err(err) = conn.send(&[packet]).await {
                        error!("error sending packets to connection {:#?}", err);
                    }
                }
            };
        });
        
        Self {
            outgoing: out_send,
            incoming: in_recv,
            conn_task,
        }
    }
    
    pub fn send(&self, packet: V944) -> anyhow::Result<()> {
        self.outgoing.try_send(packet)?;
        
        Ok(())
    }
    
    pub fn recv(&self) -> anyhow::Result<V944> {
        Ok(self.incoming.try_recv()?)
    }

    pub fn on_login_success(&mut self) {
        self.send_play_status(PlayStatus::LoginSuccess, false);
    }

    pub fn send_play_status(&mut self, status: PlayStatus, immediate: bool) {
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