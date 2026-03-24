use log::{error, info};
use std::process::exit;
use crate::config::ChorusConfig;
use crate::server::Server;

mod block;
mod config;
mod entity;
mod error;
mod level;
mod logger;
mod math;
mod network;
mod registry;
mod server;
mod utils;
mod info;

fn main() {
    let config = ChorusConfig::setup();

    logger::setup_logger(config.log_to_file, &config.logs_directory);

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(config.threads)
        .enable_all()
        .build()
        .unwrap_or_else(|err| {
            error!("Failed to create Tokio-Runtime, Err: {err:?}");
            exit(1)
        });

    runtime.block_on(async { 
        Server::
            get_mut().await
            .start().await
            .unwrap_or_else(|e| {
                error!("{}", e);
            });
        info!("Stopped.") 
    })
}
