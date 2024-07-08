use crate::error::Error;
use crate::result::cmd::CmdResult;
use serde::{Deserialize, Serialize};
use ssh2::Session;
use std::io::Read;
use std::net::TcpStream;
use std::path::PathBuf;

#[derive(Clone)]
pub struct Ssh2HostHandler {
    pub hostaddress: String,
    pub sshsession: Session,
    pub authmode: Ssh2AuthMode,
}

impl Ssh2HostHandler {
    pub fn new() -> Ssh2HostHandler {
        Ssh2HostHandler {
            hostaddress: String::new(),
            sshsession: Session::new().unwrap(),
            authmode: Ssh2AuthMode::Unset,
        }
    }

    pub fn none() -> Ssh2HostHandler {
        Ssh2HostHandler {
            hostaddress: String::from(""),
            sshsession: Session::new().unwrap(), // TODO: remove this unnecessary construction
            authmode: Ssh2AuthMode::Unset,
        }
    }

    pub fn from(hostaddress: String, authmode: Ssh2AuthMode) -> Ssh2HostHandler {
        Ssh2HostHandler {
            hostaddress,
            sshsession: Session::new().unwrap(),
            authmode,
        }
    }

    pub fn set_to(&mut self, hostaddress: String, authmode: Ssh2AuthMode) {
        self.hostaddress = hostaddress;
        self.authmode = authmode;
    }

    pub fn init(&mut self) -> Result<(), Error> {
        if self.authmode == Ssh2AuthMode::Unset {
            return Err(Error::MissingInitialization);
        } else {
            // TODO : add SSH custom port handling
            match TcpStream::connect(format!("{}:22", self.hostaddress)) {
                Ok(tcp) => {
                    self.sshsession.set_tcp_stream(tcp);
                    self.sshsession.handshake().unwrap();

                    match &self.authmode {
                        Ssh2AuthMode::UsernamePassword(credentials) => {
                            self.sshsession
                                .userauth_password(&credentials.username, &credentials.password)
                                .unwrap();
                            if self.sshsession.authenticated() {
                                return Ok(());
                            } else {
                                return Err(Error::FailedInitialization(String::from(
                                    "PLACEHOLDER",
                                )));
                            }
                        }
                        Ssh2AuthMode::SshKeys((username, privatekeypath)) => {
                            self.sshsession
                                .userauth_pubkey_file(
                                    username.as_str(),
                                    None,
                                    &privatekeypath,
                                    None,
                                )
                                .unwrap(); // TODO : add pubkey and passphrase support
                            if self.sshsession.authenticated() {
                                return Ok(());
                            } else {
                                return Err(Error::FailedInitialization(String::from(
                                    "PLACEHOLDER",
                                )));
                            }
                        }
                        Ssh2AuthMode::SshAgent(_agent) => {
                            return Ok(());
                        } // TODO
                        _ => return Err(Error::FailedInitialization(String::from("Other error"))),
                    }
                }
                Err(e) => {
                    return Err(Error::FailedTcpBinding(format!("{:?}", e)));
                }
            }
        }
    }

    pub fn is_this_cmd_available(&self, cmd: &str) -> Result<bool, Error> {
        let check_cmd_content = format!("command -v {}", cmd);
        let check_cmd_result = self.run_cmd(check_cmd_content.as_str());

        match check_cmd_result {
            Ok(cmd_result) => {
                if cmd_result.exitcode == 0 {
                    return Ok(true);
                } else {
                    return Ok(false);
                }
            }
            Err(e) => {
                return Err(Error::FailureToRunCommand(format!("{:?}", e)));
            }
        }
    }

    pub fn run_cmd(&self, cmd: &str) -> Result<CmdResult, Error> {
        assert!(
            self.authmode != Ssh2AuthMode::Unset,
            "Can't run command on remote host : authentication unset"
        );

        let mut channel = self.sshsession.channel_session().unwrap();
        channel.exec(cmd).unwrap();
        let mut s = String::new();
        channel.read_to_string(&mut s).unwrap();
        let _ = channel.wait_close();

        return Ok(CmdResult {
            exitcode: channel.exit_status().unwrap(),
            stdout: s,
        });
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Ssh2AuthMode {
    Unset,
    UsernamePassword(Credentials),
    SshKeys((String, PathBuf)), // (username, private key's path)
    SshAgent(String),           // Name of SSH agent
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Credentials {
    username: String,
    password: String,
}

impl Credentials {
    pub fn from(username: String, password: String) -> Credentials {
        Credentials { username, password }
    }
}
