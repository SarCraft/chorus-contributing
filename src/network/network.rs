use bedrockrs::network::listener::Listener;
use log::{error, info};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use bedrockrs::network::connection::Connection;
use bedrockrs::proto::{Packets, ProtoVersion, Unknown, V944};
use bevy_app::{App, FixedUpdate, Plugin, Startup};
use bevy_ecs::prelude::{Commands, Query, Res};
use bevy_ecs::resource::Resource;
use bevy_tasks::block_on;
use crossbeam_channel::Receiver;
use tokio::task::JoinHandle;
use crate::config::ChorusConfig;
use crate::network::handler::start_session_handler;
use crate::network::session::Session;

#[derive(Resource)]
pub struct NetworkState {
    incoming: Receiver<Connection<Unknown>>,
    runtime: tokio::runtime::Runtime,
    listener_task: JoinHandle<()>,
}

pub struct Network;

impl Plugin for Network {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, Network::init_network)
            .add_systems(FixedUpdate, Network::tick);
    }
}

impl Network {
    pub fn init_network(config: Res<ChorusConfig>, mut commands: Commands) {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(config.threads)
            .enable_all()
            .build()
            .unwrap();
        
        let mut listener = runtime.block_on(async {
            let mut listener = Listener::new_raknet(
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
            .await.unwrap();
            
            listener.start().await.unwrap();
            listener
        });
        
        let (incoming_send, incoming_recv) = crossbeam_channel::unbounded();
        
        let listener_task = runtime.spawn(async move {
            loop {
                let conn = listener.accept().await.unwrap();

                info!("Connected: {}", conn.get_socket_addr().ip().to_string());

                incoming_send.send(conn).unwrap();
            }
        });
        
        commands.insert_resource(NetworkState {
            incoming: incoming_recv,
            runtime,
            listener_task,
        })
    }
    
    pub fn tick(
        network: Res<NetworkState>,
        query: Query<&Session>,
        mut commands: Commands,
    ) {
        for conn in network.incoming.try_iter() {
            commands.spawn(Session::new(conn, &network.runtime));
        }
        
        for session in query.iter() {
            while let Ok(packet) = session.recv() {
                start_session_handler::handle(session, &packet);
                
                info!("Packet({:?}): {:?}", packet.id(), packet);
            }
        }
    }
}
