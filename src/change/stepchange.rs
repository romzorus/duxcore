use crate::connection::hosthandler::HostHandler;
use crate::result::apicallresult::ApiCallResult;
use crate::result::stepresult::StepResult;
use crate::task::moduleblock::Apply;
use crate::task::moduleblock::ModuleApiCall;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StepChange {
    AllowedFailure(String),
    AlreadyMatched(String),
    FailedToEvaluate(String), // The module can't work on this host (trying to use yum/dnf on Debian for example)
    ModuleApiCalls(Vec<ModuleApiCall>),
}

impl StepChange {
    pub fn matched(message: &str) -> StepChange {
        StepChange::AlreadyMatched(message.to_string())
    }

    pub fn failed_to_evaluate(message: &str) -> StepChange {
        StepChange::FailedToEvaluate(message.to_string())
    }

    pub fn changes(changes: Vec<ModuleApiCall>) -> StepChange {
        StepChange::ModuleApiCalls(changes)
    }

    pub fn display(&self) -> Vec<String> {
        match self {
            StepChange::AllowedFailure(message) => {
                return Vec::from([message.clone()]);
            }
            StepChange::AlreadyMatched(message) => {
                return Vec::from([message.clone()]);
            }
            StepChange::FailedToEvaluate(message) => {
                return Vec::from([message.clone()]);
            }
            StepChange::ModuleApiCalls(changeslist) => {
                let mut display_contents: Vec<String> = Vec::new();
                for change in changeslist {
                    let apicalldisplay = match change {
                        ModuleApiCall::None(message) => message.clone(),
                        // **BEACON_1**
                        ModuleApiCall::LineInFile(block) => block.display(),
                        ModuleApiCall::Command(block) => block.display(),
                        ModuleApiCall::Apt(block) => block.display(),
                        ModuleApiCall::Ping(block) => block.display(),
                        ModuleApiCall::YumDnf(block) => block.display(),
                    };
                    display_contents.push(apicalldisplay);
                }
                return display_contents;
            }
        }
    }

    pub fn apply_moduleblockchange(&self, hosthandler: &mut HostHandler) -> StepResult {
        match self {
            StepChange::AllowedFailure(_message) => return StepResult::none(),
            StepChange::AlreadyMatched(_message) => return StepResult::none(),
            StepChange::FailedToEvaluate(_message) => return StepResult::none(),
            StepChange::ModuleApiCalls(changeslist) => {
                let mut results: Vec<ApiCallResult> = Vec::new();
                for change in changeslist {
                    let apicallresult = match change {
                        ModuleApiCall::None(_) => ApiCallResult::none(),
                        // **BEACON_2**
                        ModuleApiCall::LineInFile(block) => {
                            block.apply_moduleblock_change(hosthandler)
                        }
                        ModuleApiCall::Command(block) => {
                            block.apply_moduleblock_change(hosthandler)
                        }
                        ModuleApiCall::Apt(block) => block.apply_moduleblock_change(hosthandler),
                        ModuleApiCall::Ping(block) => block.apply_moduleblock_change(hosthandler),
                        ModuleApiCall::YumDnf(block) => block.apply_moduleblock_change(hosthandler),
                    };
                    results.push(apicallresult);
                }
                return StepResult::from(results);
            }
        }
    }
}
