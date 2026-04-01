use crate::server::config::RakServerConfig;
use crate::server::internal::RakServerInternal;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::mpsc::{UnboundedReceiver, unbounded_channel};
use tokio::sync::{Mutex, Notify, RwLock};

mod config;
mod internal;

#[derive(Clone)]
pub struct RakServer {
    addr: SocketAddr,

    internal: Arc<RwLock<RakServerInternal>>,
    out_rx: Arc<Mutex<UnboundedReceiver<(Vec<u8>, SocketAddr)>>>,

    config: Arc<RwLock<RakServerConfig>>,

    started_notify: Arc<Notify>,
    stopped_notify: Arc<Notify>,
}

impl RakServer {
    pub async fn new<F>(addr: SocketAddr, conf: F) -> Self
    where
        F: FnOnce(&mut RakServerConfig),
    {
        let mut config = RakServerConfig::default();
        conf(&mut config);
        let config = Arc::new(RwLock::new(config));

        let (tx, rx) = unbounded_channel::<(Vec<u8>, SocketAddr)>();

        Self {
            addr,

            internal: Arc::new(RwLock::new(RakServerInternal::new(
                config.clone(),
                addr,
                tx,
            ))),
            out_rx: Arc::new(Mutex::new(rx)),

            config: config.clone(),

            started_notify: Arc::new(Notify::new()),
            stopped_notify: Arc::new(Notify::new()),
        }
    }

    pub async fn start(&mut self, block: bool) -> &mut Self {
        let server_task = tokio::spawn({
            let addr = self.addr;
            let config = self.config.clone();
            let internal = self.internal.clone();
            let out_rx = self.out_rx.clone();

            let started_notify = self.started_notify.clone();
            let stopped_notify = self.stopped_notify.clone();

            async move {
                let socket = Arc::new(UdpSocket::bind(addr).await.unwrap());

                tokio::spawn({
                    let stopped_notify = stopped_notify.clone();
                    let socket = socket.clone();
                    let mtu = config.read().await.max_mtu_size;
                    async move {
                        let mut buf = vec![0u8; mtu as usize];

                        loop {
                            tokio::select! {
                                _ = stopped_notify.notified() => { break; }
                                recv = socket.recv_from(&mut buf) => {
                                    if let Ok((len, addr)) = recv {
                                        internal.write().await.handle(&buf[..len], addr).await;
                                    }
                                }
                            }
                        }
                    }
                });

                tokio::spawn({
                    let stopped_notify = stopped_notify.clone();
                    let socket = socket.clone();
                    let rx = out_rx.clone();
                    async move {
                        let mut rx = rx.lock().await;
                        loop {
                            tokio::select! {
                                _ = stopped_notify.notified() => { break; }
                                packet = rx.recv() => {
                                    if let Some(packet) = packet {
                                        socket.send_to(&packet.0, &packet.1).await.unwrap();
                                    }
                                }
                            }
                        }
                    }
                });

                started_notify.notify_waiters();
                stopped_notify.notified().await;
            }
        });

        self.started_notify.notified().await;
        if block {
            server_task.await.unwrap();
        }
        self
    }

    pub async fn stop(&mut self) {
        self.stopped_notify.notify_waiters();
    }
}
