use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum AuthType {
    Online = 0,
    Guest = 1,
    Offline = 2,
}