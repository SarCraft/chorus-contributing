use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum AuthType {
    Full,
    Guest,
    SelfSigned,
}