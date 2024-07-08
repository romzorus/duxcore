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
