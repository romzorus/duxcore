use serde::{Deserialize, Serialize};

use crate::change::stepchange::StepChange;
use crate::connection::hosthandler::HostHandler;
use crate::connection::specification::Privilege;
use crate::error::Error;
use crate::modules::prelude::*;
use crate::result::apicallresult::ApiCallResult;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ModuleBlockExpectedState {
    None, // Used for new() methods, initializations and errors
    // **BEACON_2**
    Service(ServiceBlockExpectedState),
    LineInFile(LineInFileBlockExpectedState),
    Command(CommandBlockExpectedState),
    Apt(AptBlockExpectedState),
    Dnf(YumDnfBlockExpectedState),
    Ping(PingBlockExpectedState),
    Yum(YumDnfBlockExpectedState),
}

impl ModuleBlockExpectedState {
    pub fn new() -> ModuleBlockExpectedState {
        ModuleBlockExpectedState::None
    }

    pub fn dry_run_moduleblock(
        &self,
        hosthandler: &mut HostHandler,
        privilege: Privilege,
        allowed_to_fail: bool,
    ) -> Result<(StepChange, bool), Error> {
        let mbchange_result: Result<StepChange, Error> = match &self {
            ModuleBlockExpectedState::None => Ok(StepChange::matched("none")),
            // **BEACON_3**
            ModuleBlockExpectedState::Service(block) => block.dry_run_block(hosthandler, privilege),
            ModuleBlockExpectedState::LineInFile(block) => {
                block.dry_run_block(hosthandler, privilege)
            }
            ModuleBlockExpectedState::Command(block) => block.dry_run_block(hosthandler, privilege),
            ModuleBlockExpectedState::Apt(block) => block.dry_run_block(hosthandler, privilege),
            ModuleBlockExpectedState::Dnf(block) => block.dry_run_block(hosthandler, privilege),
            ModuleBlockExpectedState::Ping(block) => block.dry_run_block(hosthandler, privilege),
            ModuleBlockExpectedState::Yum(block) => block.dry_run_block(hosthandler, privilege),
        };

        match mbchange_result {
            Ok(mbchange) => {
                return Ok((mbchange, allowed_to_fail));
            }
            Err(error_content) => match error_content {
                Error::FailedDryRunEvaluation(message) => {
                    if allowed_to_fail {
                        return Ok((StepChange::AllowedFailure(message), allowed_to_fail));
                    } else {
                        return Err(Error::FailedTaskDryRun(message));
                    }
                }
                _ => {
                    // return Err(Error::AnyOtherError(
                    //     "Module returned wrong Error type".to_string(),
                    // ))
                    return Err(Error::AnyOtherError(
                        format!("Wrong error type : {:?}", error_content)
                    ));
                }
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModuleApiCall {
    None(String),
    // **BEACON_4**
    Service(ServiceApiCall),
    LineInFile(LineInFileApiCall),
    Command(CommandApiCall),
    Apt(AptApiCall),
    Ping(PingApiCall),
    YumDnf(YumDnfApiCall),
}

pub trait DryRun {
    fn dry_run_block(
        &self,
        hosthandler: &mut HostHandler,
        privilege: Privilege,
    ) -> Result<StepChange, Error>;
}

pub trait Apply {
    fn display(&self) -> String;
    fn apply_moduleblock_change(&self, hosthandler: &mut HostHandler) -> ApiCallResult;
}
