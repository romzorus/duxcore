use crate::change::changelist::ChangeList;
use crate::change::stepchange::StepChange;
use crate::connection::hosthandler::HostHandler;
use crate::connection::hosthandler::HostHandlingInfo;
use crate::error::Error;
use crate::result::apicallresult::ApiCallStatus;
use crate::result::resultlist::ResultList;
use crate::task::moduleblock::ModuleApiCall;
use crate::task::tasklist::RunningMode;
use crate::task::tasklist::TaskList;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assignment {
    pub correlationid: String,
    pub runningmode: RunningMode,
    pub host: String,
    pub hosthandlinginfo: HostHandlingInfo,
    pub variables: HashMap<String, String>,
    pub tasklist: TaskList,
    pub changelist: ChangeList,
    pub resultlist: ResultList,
    pub finalstatus: AssignmentFinalStatus,
}

impl Assignment {
    pub fn new(correlationid: String) -> Assignment {
        Assignment {
            correlationid,
            runningmode: RunningMode::DryRun, // DryRun is default running mode
            host: String::from(""),
            hosthandlinginfo: HostHandlingInfo::new(),
            variables: HashMap::new(),
            tasklist: TaskList::new(),
            changelist: ChangeList::new(),
            resultlist: ResultList::new(),
            finalstatus: AssignmentFinalStatus::Unset,
        }
    }

    pub fn from(
        correlationid: String,
        runningmode: RunningMode,
        host: String,
        hosthandlinginfo: HostHandlingInfo,
        variables: HashMap<String, String>,
        tasklist: TaskList,
        changelist: ChangeList,
        resultlist: ResultList,
        finalstatus: AssignmentFinalStatus,
    ) -> Assignment {
        Assignment {
            correlationid,
            runningmode,
            host,
            hosthandlinginfo,
            variables,
            tasklist,
            changelist,
            resultlist,
            finalstatus,
        }
    }

    pub fn dry_run(&mut self, hosthandler: &mut HostHandler) -> Result<(), Error> {
        match self
            .tasklist
            .dry_run_tasklist(hosthandler)
        {
            Ok(changelist) => {
                match &changelist.taskchanges {
                    Some(taskchangelist) => {
                        let mut finalstatus = AssignmentFinalStatus::AlreadyMatched;
                        for taskchange in taskchangelist {
                            for step in taskchange.stepchanges.clone() {
                                if let StepChange::ModuleApiCalls(apicalllist) = step {
                                    for apicall in apicalllist {
                                        match apicall {
                                            ModuleApiCall::None(_) => {}
                                            _ => {
                                                finalstatus = AssignmentFinalStatus::Unset;
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        self.finalstatus = finalstatus;
                    }
                    None => {}
                }
                self.changelist = changelist;
                return Ok(());
            }
            Err(e) => {
                if let Error::FailedTaskDryRun(message) = &e {
                    self.finalstatus = AssignmentFinalStatus::FailedDryRun(message.clone());
                }
                return Err(e);
            }
        }
    }

    // TODO : allow direct run with this method
    pub fn apply(&mut self, hosthandler: &mut HostHandler) -> Result<(), Error> {
        if let RunningMode::Apply = self.runningmode {
            if let AssignmentFinalStatus::Unset = self.finalstatus {
                let tasklistresult = self.changelist.apply_changelist(hosthandler);
                // "Save" the results
                self.resultlist = tasklistresult.clone();

                // Decide on the final status of the Assignment based on all the results
                // -> Considered successfull unless it failed at some point
                self.finalstatus = AssignmentFinalStatus::Changed;
                for taskresult in tasklistresult.taskresults.iter() {
                    for stepresult in taskresult.stepresults.as_ref().unwrap().iter() {
                        for apicallresult in stepresult.apicallresults.iter() {
                            match apicallresult.status {
                                ApiCallStatus::Failure(_) => {
                                    self.finalstatus = AssignmentFinalStatus::FailedChange;
                                    break;
                                }
                                ApiCallStatus::AllowedFailure(_) => {
                                    self.finalstatus = AssignmentFinalStatus::ChangedWithFailures;
                                    break;
                                }
                                _ => {}
                            }
                        }
                    }
                }
            } else {
                return Err(Error::WrongInitialization);
            }
        } else {
            return Err(Error::WrongInitialization);
        }

        Ok(())
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum AssignmentFinalStatus {
    Unset,
    AlreadyMatched,
    FailedDryRun(String),
    Changed,
    ChangedWithFailures,
    FailedChange,
    GenericFailed(String),
}
