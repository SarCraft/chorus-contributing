use crate::config::Config;
use crate::logger::setup_logger;
use crate::server::Server;
use bevy_app::{App, PreStartup, ScheduleRunnerPlugin, TaskPoolOptions, TaskPoolPlugin};
use bevy_time::TimePlugin;

mod block;
mod config;
mod entity;
mod error;
mod info;
mod level;
mod logger;
mod math;
mod network;
mod registry;
mod server;
mod utils;

fn main() {
    let config = Config::setup();

    App::new()
        .add_plugins(TimePlugin)
        .add_plugins(ScheduleRunnerPlugin::default())
        .add_plugins(TaskPoolPlugin {
            task_pool_options: TaskPoolOptions {
                max_total_threads: config.threads,
                min_total_threads: config.threads,
                ..Default::default()
            },
        })
        .insert_resource(config)
        .add_systems(PreStartup, setup_logger)
        .add_plugins(Server)
        .run();
}
