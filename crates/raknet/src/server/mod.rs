use crate::server::config::RakServerConfig;

mod config;

pub struct RakServer {
    host: String,
    port: u16,
    config: RakServerConfig,
}

impl Default for RakServer {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 19132,
            config: RakServerConfig::default(),
        }
    }
}

impl RakServer {
    fn new<F>(host: &str, port: u16, configure: F) -> Self
    where F: FnOnce(&mut RakServerConfig) {
        Self {
            host: host.to_string(),
            port,
            config: {
                let mut config = RakServerConfig::default();
                configure(&mut config);
                config
            }
        }
    }
}