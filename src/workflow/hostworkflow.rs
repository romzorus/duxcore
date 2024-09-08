use crate::workflow::taskflow::{TaskFlow, TaskStatus};
use crate::task::tasklist::TaskList;
use crate::connection::hosthandler::HostHandler;
use std::collections::HashMap;
use crate::host::hosts::Host;
use crate::error::Error;

#[derive(Debug, Clone)]
pub struct HostWorkFlow {
    pub task_flows: Vec<TaskFlow>,
    pub final_status: HostWorkFlowStatus,
    pub context: Context,
}

impl HostWorkFlow {
    pub fn new() -> HostWorkFlow {
        HostWorkFlow {
            task_flows: Vec::new(),
            final_status: HostWorkFlowStatus::NotRunYet,
            context: Context::new(),
        }
    }

    pub fn from(task_list: TaskList, context: Context) -> HostWorkFlow {
        let mut task_flows: Vec<TaskFlow> = Vec::new();

        for task_block in task_list.tasks.iter() {
            task_flows.push(TaskFlow::from(task_block.clone()))
        }

        HostWorkFlow {
            task_flows,
            final_status: HostWorkFlowStatus::NotRunYet,
            context,
        }
    }

    pub fn dry_run(&mut self, hosthandler: &mut HostHandler) -> Result<(), Error> {
        let mut changes_required = false;

        for task_flow in self.task_flows.iter_mut() {
            match task_flow.dry_run(hosthandler) {
                Ok(()) => {
                    if let TaskStatus::ChangeRequired = task_flow.task_status {
                        changes_required = true;
                    }
                }
                Err(error) => {
                    return Err(error);
                }
            }
        }

        if changes_required {
            self.final_status = HostWorkFlowStatus::ChangeRequired;
        } else {
            self.final_status = HostWorkFlowStatus::AlreadyMatched;
        }

        Ok(())
    }
    pub fn apply(&mut self, hosthandler: &mut HostHandler) -> Result<(), Error> {

        for task_flow in self.task_flows.iter_mut() {
            match task_flow.apply(hosthandler, &mut self.context) {
                Ok(()) => {}
                Err(error) => {
                    return Err(error);
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum HostWorkFlowStatus {
    NotRunYet,
    AlreadyMatched,
    ChangeRequired,
    ApplySuccesful,
    ApplyWithAllowedFailure,
    ApplyFailed,
}

#[derive(Clone, Debug)]
pub struct Context {
    pub vars: HashMap<String, String>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            vars: HashMap::new(),
        }
    }

    pub fn from(host: Host) -> Context {
        match host.vars {
            Some(vars) => Context { vars },
            None => Context {
                vars: HashMap::new(),
            },
        }
    }
}
