use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionMode {
    Unset,
    LocalHost,
    Ssh2,
    // Ssh3
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Privilege {
    Usual,          // Run cmd as the current authenticated user
    WithSudo,       // Run cmd with sudo
    AsUser(String), // Run cmd as another user
}

// Message broker (RabbitMQ) part
// TODO: create a dedicated module for this ?

pub const REFRESH_INTERVAL_MILLI_SECONDS: u64 = 300;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

impl Credentials {
    pub fn from(username: String, password: String) -> Credentials {
        Credentials { username, password }
    }
}
