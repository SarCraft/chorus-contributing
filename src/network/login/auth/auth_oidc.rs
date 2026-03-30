use bevy_ecs::prelude::Resource;
use jsonwebtoken::jwk::JwkSet;

#[derive(Resource, Clone, Debug)]
pub struct AuthOIDC {
    pub issuer: String,
    pub jwks: JwkSet,
    pub audience: String,
}

impl AuthOIDC {
    const DISCOVERY: &str = "https://client.discovery.minecraft-services.net/api/v1.0/discovery/MinecraftPE/builds/1.0.0.0";
    const AUDIENCE: &str = "api://auth-minecraft-services/multiplayer";
    
    pub fn fetch() -> Option<Self> {
        let discovery: serde_json::Value = reqwest::blocking::get(Self::DISCOVERY)
            .ok()?
            .json()
            .ok()?;
        
        let base = discovery["result"]["serviceEnvironments"]["auth"]["prod"]["serviceUri"]
            .as_str()?
            .to_string();
        
        let config_url = format!("{}/.well-known/openid-configuration", base);
        let config: serde_json::Value = reqwest::blocking::get(&config_url)
            .ok()?
            .json()
            .ok()?;

        let issuer = config["issuer"].as_str()?.to_string();
        
        let jwks_uri = config["jwks_uri"].as_str()?;
        let jwks: JwkSet = reqwest::blocking::get(jwks_uri)
            .ok()?
            .json()
            .ok()?;

        Some(Self {
            issuer,
            jwks,
            audience: Self::AUDIENCE.to_string(),
        })
    }
}