use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::Notify;
use tokio::task::JoinHandle;
use crate::server::config::RakServerConfig;
use crate::util::constants;

mod config;

pub struct RakServer {
    host: String,
    port: u16,
    config: RakServerConfig,
    
    started_notify: Arc<Notify>,
    stopped_notify: Arc<Notify>,
}

impl RakServer {
    pub fn new<F>(host: &str, port: u16, conf: F) -> Self
    where 
        F: FnOnce(&mut RakServerConfig),
    {
        Self {
            host: host.to_string(),
            port,
            config: {
                let mut config = RakServerConfig::default();
                conf(&mut config);
                config
            },
            
            started_notify: Arc::new(Notify::new()),
            stopped_notify: Arc::new(Notify::new()),
        }
    }
    
    pub async fn start(&mut self, block: bool) -> &mut Self {
        let server_task = tokio::spawn({
            let host = self.host.clone();
            let port = self.port;
            let started_notify = self.started_notify.clone();
            let stopped_notify = self.stopped_notify.clone();

            async move {
                let mut socket = Arc::new(UdpSocket::bind((host, port)).await.unwrap());

                let accept_task = tokio::spawn({
                    let socket = socket.clone();
                    async move {
                        let mut buf = [0u8; constants::MAX_MTU_SIZE as usize];

                        loop {
                            tokio::select! {
                            _ = stopped_notify.notified() => { break; }
                            recv = socket.recv_from(&mut buf) => {
                                
                            }
                        }
                        }
                    }
                });

                started_notify.notify_waiters();
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
    
    fn handle(buf: &[u8]) {
        
    }
    
    fn test() {
        let srv = RakServer::new("0.0.0.0", 19132, |cfg| {
            cfg.message = vec![0, 0, 0, 0]
        });
    }
}