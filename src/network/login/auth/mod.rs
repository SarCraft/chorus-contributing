use crate::network::login::auth::auth_oidc::AuthOIDC;
use bevy_app::{App, Plugin, Startup};
use bevy_ecs::prelude::Commands;
use tracing::debug;

pub mod auth_identity;
pub mod auth_oidc;
pub mod auth_payload;
pub mod auth_type;

pub struct LoginAuthOIDC;

impl Plugin for LoginAuthOIDC {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::fetch_oidc);
    }
}

impl LoginAuthOIDC {
    pub fn fetch_oidc(mut commands: Commands) {
        if let Some(oidc) = AuthOIDC::fetch() {
            debug!("Auth OIDC fetch succeeded");
            commands.insert_resource(oidc)
        } else {
            debug!("Auth OIDC fetch failed")
        }
    }
}
