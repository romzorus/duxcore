use crate::connection::connectionmode::localhost::{LocalHostConnectionDetails, LocalHostHandler};
use crate::connection::connectionmode::ssh2mode::{Ssh2ConnectionDetails, Ssh2HostHandler};
use crate::connection::specification::{ConnectionMode, Privilege};
use crate::error::Error;
use crate::result::cmd::CmdResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostHandlingInfo {
    pub connectionmode: ConnectionMode,
    pub hostaddress: String,
    pub connectiondetails: ConnectionDetails,
}

impl HostHandlingInfo {
    pub fn new() -> HostHandlingInfo {
        HostHandlingInfo {
            connectionmode: ConnectionMode::Unset,
            hostaddress: String::new(),
            connectiondetails: ConnectionDetails::Unset,
        }
    }

    pub fn from(
        connectionmode: ConnectionMode,
        hostaddress: String,
        connectiondetails: ConnectionDetails,
    ) -> HostHandlingInfo {
        HostHandlingInfo {
            connectionmode,
            hostaddress: hostaddress.clone(),
            connectiondetails,
        }
    }
}

#[derive(Clone)]
pub struct HostHandler {
    connectionmode: ConnectionMode,
    localhost: Option<LocalHostHandler>,
    ssh2: Option<Ssh2HostHandler>,
}

impl HostHandler {
    pub fn from(hosthandlinginfo: &HostHandlingInfo) -> Result<HostHandler, Error> {
        match hosthandlinginfo.connectionmode {
            ConnectionMode::Unset => Err(Error::MissingInitialization),
            ConnectionMode::LocalHost => {
                if let ConnectionDetails::LocalHost(localhostconnectiondetails) =
                    &hosthandlinginfo.connectiondetails
                {
                    Ok(HostHandler {
                        connectionmode: hosthandlinginfo.connectionmode.clone(),
                        localhost: Some(LocalHostHandler::from(
                            localhostconnectiondetails.user.clone(),
                        )),
                        ssh2: None,
                    })
                } else {
                    Err(Error::WrongInitialization)
                }
            }
            ConnectionMode::Ssh2 => {
                if let ConnectionDetails::Ssh2(ss2connectiondetails) =
                    &hosthandlinginfo.connectiondetails
                {
                    Ok(HostHandler {
                        connectionmode: hosthandlinginfo.connectionmode.clone(),
                        localhost: None,
                        ssh2: Some(Ssh2HostHandler::from(
                            ss2connectiondetails.hostaddress.clone(),
                            ss2connectiondetails.authmode.clone(),
                        )),
                    })
                } else {
                    Err(Error::WrongInitialization)
                }
            }
        }
    }

    pub fn init(&mut self) -> Result<(), Error> {
        match self.connectionmode {
            ConnectionMode::Unset => {
                return Err(Error::MissingInitialization);
            }
            // Nothing to initialize when working on localhost
            ConnectionMode::LocalHost => {
                return Ok(());
            }
            ConnectionMode::Ssh2 => self.ssh2.as_mut().unwrap().init(),
        }
    }

    // Use this to check if a command is available on target host
    pub fn is_this_cmd_available(&mut self, cmd: &str) -> Result<bool, Error> {
        match self.connectionmode {
            ConnectionMode::Unset => Err(Error::MissingInitialization),
            ConnectionMode::LocalHost => {
                self.localhost.as_mut().unwrap().is_this_cmd_available(cmd)
            }
            ConnectionMode::Ssh2 => self.ssh2.as_mut().unwrap().is_this_cmd_available(cmd),
        }
    }

    pub fn run_cmd(&mut self, cmd: &str, privilege: Privilege) -> Result<CmdResult, Error> {
        let final_cmd = final_cmd(cmd.to_string(), privilege.clone());
        match self.connectionmode {
            ConnectionMode::Unset => Err(Error::MissingInitialization),
            ConnectionMode::LocalHost => {
                self.localhost.as_mut().unwrap().run_cmd(final_cmd.as_str())
            }
            ConnectionMode::Ssh2 => self.ssh2.as_mut().unwrap().run_cmd(final_cmd.as_str()),
        }
    }
}

// TODO : add some syntax checks
fn final_cmd(cmd: String, privilege: Privilege) -> String {
    match privilege {
        Privilege::Usual => {
            return format!("{} 2>&1", cmd);
        }
        Privilege::WithSudo => {
            let final_cmd = format!("sudo -u root {} 2>&1", cmd);
            return final_cmd;
        }
        Privilege::AsUser(username) => {
            let final_cmd = format!("sudo -u {} {} 2>&1", username, cmd);
            return final_cmd;
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionDetails {
    Unset,
    LocalHost(LocalHostConnectionDetails),
    Ssh2(Ssh2ConnectionDetails),
}
