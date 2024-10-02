use crate::connection::hosthandler::HostHandler;
use crate::connection::hosthandler::HostHandlingInfo;
use crate::error::Error;
use crate::result::apicallresult::ApiCallStatus;
use crate::task::moduleblock::ModuleApiCall;
use crate::task::tasklist::RunningMode;
use crate::task::tasklist::TaskList;
use crate::workflow::hostworkflow::HostWorkFlow;
use crate::workflow::hostworkflow::HostWorkFlowStatus;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A 'DuxJob' is a work-in-progress T. A DuxJob can be built from scractch or from an Assignment.
#[derive(Debug, Clone)]
pub struct DuxJob {
    correlationid: String,
    runningmode: RunningMode,
    host_workflow: HostWorkFlow,
    hosthandler: HostHandler
}

impl DuxJob {
    pub fn from_assignment(assignment: Assignment) -> DuxJob {
        DuxJob {
            correlationid: assignment.correlationid,
            runningmode: assignment.runningmode,
            host_workflow: HostWorkFlow::from(
                assignment.tasklist,
                DuxContext::from(assignment.variables)
            ),
            hosthandler: HostHandler::from(assignment.hosthandlinginfo).unwrap()
        }
    }

    pub fn dry_run(&mut self) -> Result<(), Error> {
        self.host_workflow.dry_run(self.hosthandler)
    }

    // pub fn apply(&mut self, )
}

/// An 'Assignment' withholds everything required to run a TaskList on a given Host (expected state, host information, variables...). An Assignment is serializable/deserializable, meaning it can be sent over any protocol which allows sending arbitrary data (HTTP, MQTT, AMQP, gPRC...), which is exactly the point of the Assignment. It is what you build and send to a "worker node" when you want to distribute the work load. The worker node will then build a Job upon the received Assignment and run this Job.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assignment {
    pub correlationid: String,
    pub runningmode: RunningMode,
    pub host: String,
    pub hosthandlinginfo: HostHandlingInfo,
    pub variables: Option<HashMap<String, String>>,
    pub tasklist: TaskList,
    pub finalstatus: HostWorkFlowStatus,
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
