use std::error::Error;
use bedrockrs::network::error::ListenerError;
use bedrockrs::network::listener::Listener;
use log::{error, info};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use std::sync::Arc;
use bedrockrs::proto::{ProtoVersion, V944};
use tokio::sync::Mutex;
use crate::config::ChorusConfig;
use crate::network::session::Session;

pub struct Network {
    listener: Arc<Mutex<Listener>>,
    sessions: Arc<Mutex<Vec<Arc<Mutex<Session>>>>>,
}

impl Network {
    pub async fn default(config: &ChorusConfig) -> Self {
        Self {
            listener: Arc::new(Mutex::new(
                Listener::new_raknet(
                    SocketAddr::new(
                        IpAddr::V4(
                            Ipv4Addr::from_str(config.ip.as_str()).unwrap_or_else(
                                |err| {
                                    error!("{}: {}", err, config.ip);

                                    Ipv4Addr::UNSPECIFIED
                                },
                            ),
                        ),
                        config.port.clone(),
                    ),
                    config.name.clone(),
                    config.sub_name.clone(),
                    String::from(V944::GAME_VERSION),
                    V944::PROTOCOL_VERSION,
                    V944::RAKNET_VERSION,
                    config.max_players.clone(),
                    0,
                    false,
                )
                .await
                .unwrap(),
            )),
            sessions: Arc::new(Mutex::new(Vec::new()))
        }
    }

    pub async fn start(&mut self) -> Result<(), ListenerError> {
        self.listener.lock().await.start().await?;

        tokio::spawn({
            let listener = self.listener.clone();
            let sessions = self.sessions.clone();
            async move {
                loop {
                    let conn = listener.lock().await.accept().await.unwrap();
                    
                    info!("Connected: {}", conn.get_socket_addr().ip().to_string());
                    
                    sessions.lock().await.push(
                        Arc::new(
                            Mutex::new(
                                Session::new(conn)
                            )
                        )
                    );
                }
            }
        });

        Ok(())
    }
    
    pub async fn tick(&mut self) -> Result<(), Box<dyn Error>> {
        for session in self.sessions.lock().await.iter_mut() {
            let mut session = session.lock().await;
            
            if session.is_closed() { continue; }
            
            match session.tick().await {
                Ok(_) => {}
                Err(err) => { 
                    error!("Closing session! Cause: {:?}", err);
                    session.close(None).await;
                }
            }
        }
        
        Ok(())
    }
}
