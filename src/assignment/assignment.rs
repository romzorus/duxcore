use crate::change::changelist::ChangeList;
use crate::change::stepchange::StepChange;
use crate::connection::connectionmode::ssh2mode::Ssh2AuthMode;
use crate::connection::hosthandler::HostHandler;
use crate::connection::specification::ConnectionMode;
use crate::error::Error;
use crate::result::apicallresult::ApiCallStatus;
use crate::result::resultlist::ResultList;
use crate::task::moduleblock::ModuleApiCall;
use crate::task::tasklist::RunningMode;
use crate::task::tasklist::TaskList;
use crate::connection::hosthandler::HostHandlingInfo;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assignment {
    pub correlationid: String,
    pub runningmode: RunningMode,
    pub host: String,
    // pub connectionmode: ConnectionMode,
    // pub authmode: Ssh2AuthMode,
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
            // connectionmode: ConnectionMode::Unset,
            // authmode: Ssh2AuthMode::Unset,
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
        // connectionmode: ConnectionMode,
        // authmode: Ssh2AuthMode,
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
            // connectionmode,
            // authmode,
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
            .dry_run_tasklist(self.correlationid.clone(), hosthandler)
        {
            Ok(changelist) => {
                match &changelist.taskchanges {
                    Some(taskchangelist) => {
                        let mut finalstatus = AssignmentFinalStatus::AlreadyMatched;
                        for taskchange in taskchangelist {
                            for step in taskchange.stepchanges.clone() {
                                match step {
                                    StepChange::FailedToEvaluate(e) => {
                                        finalstatus = AssignmentFinalStatus::FailedDryRun(e);
                                        break;
                                    }
                                    StepChange::ModuleApiCalls(apicalllist) => {
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
                                    _ => {}
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
            Err(e) => match &e {
                Error::FailedTaskDryRun(message) => {
                    self.finalstatus = AssignmentFinalStatus::FailedDryRun(message.clone());
                    return Err(e);
                }
                _ => {
                    return Err(e);
                }
            },
        }
    }

    // TODO : allow direct run with this method
    pub fn apply(&mut self, hosthandler: &mut HostHandler) {
        assert_eq!(self.runningmode, RunningMode::Apply);
        assert_eq!(self.finalstatus, AssignmentFinalStatus::Unset);

        let tasklistresult = self.changelist.apply_changelist(hosthandler);
        // "Save" the results
        self.resultlist = tasklistresult.clone();

        // Decide on the final status of the Assignment based on all the results
        // -> Considered successfull unless it failed at some point
        let mut finalstatus = AssignmentFinalStatus::Changed;
        for taskresult in tasklistresult.taskresults.iter() {
            for stepresult in taskresult.stepresults.as_ref().unwrap().iter() {
                for apicallresult in stepresult.apicallresults.iter() {
                    match apicallresult.status {
                        ApiCallStatus::Failure(_) => {
                            finalstatus = AssignmentFinalStatus::FailedChange;
                        }
                        ApiCallStatus::AllowedFailure(_) => {
                            finalstatus = AssignmentFinalStatus::ChangedWithFailures;
                        }
                        _ => {}
                    }
                }
            }
        }
        self.finalstatus = finalstatus;
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
}
