use crate::change::stepchange::StepChange;
use crate::connection::hosthandler::HostHandler;
use crate::result::apicallresult::ApiCallStatus;
use crate::result::stepresult::StepResult;
use crate::result::taskresult::TaskResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskChange {
    pub stepchanges: Vec<StepChange>,
    pub allowed_failures: Vec<bool>,
}

impl TaskChange {
    pub fn new() -> TaskChange {
        TaskChange {
            stepchanges: Vec::new(),
            allowed_failures: Vec::new(),
        }
    }

    pub fn from(stepchanges: Vec<StepChange>, allowed_failures: Vec<bool>) -> TaskChange {
        TaskChange {
            stepchanges,
            allowed_failures,
        }
    }

    pub fn apply_taskchange(&self, hosthandler: &mut HostHandler) -> TaskResult {
        let mut stepresults: Vec<StepResult> = Vec::new();

        for (mbindex, moduleblockchange) in self.stepchanges.iter().enumerate() {
            let mut moduleblockresultlist = moduleblockchange.apply_moduleblockchange(hosthandler);

            // Change Failures into AllowedFailures before pushing to stepresults
            // It is done at this level and not at module level so modules don't have to bother with upper level logic.
            // We just want modules to return Failures when they fail, nothing more.
            if self.allowed_failures[mbindex] {
                for (index, apicallresult) in moduleblockresultlist
                    .apicallresults
                    .clone()
                    .iter()
                    .enumerate()
                {
                    if let ApiCallStatus::Failure(message) = &apicallresult.status {
                        moduleblockresultlist.apicallresults[index].status =
                            ApiCallStatus::AllowedFailure(message.to_string());
                    }
                }
                stepresults.push(moduleblockresultlist.clone());
            } else {
                stepresults.push(moduleblockresultlist.clone());
                // If a failure is encountered in a step, stop the "apply" there.
                if !self.allowed_failures[mbindex] {
                    for apicallresult in moduleblockresultlist.apicallresults.into_iter() {
                        if let ApiCallStatus::Failure(_) = apicallresult.status {
                            return TaskResult::from(Some(stepresults));
                        }
                    }
                }
            }
        }
        return TaskResult::from(Some(stepresults));
    }
}
