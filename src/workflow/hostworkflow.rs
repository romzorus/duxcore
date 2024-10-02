use crate::workflow::taskflow::{TaskFlow, TaskStatus};
use crate::task::tasklist::TaskList;
use crate::connection::hosthandler::HostHandler;
use std::collections::HashMap;
use crate::host::hosts::Host;
use crate::error::Error;
use serde::{Deserialize, Serialize};
use tera::Context;

#[derive(Debug, Clone)]
pub struct HostWorkFlow {
    pub task_flows: Vec<TaskFlow>,
    pub final_status: HostWorkFlowStatus,
    pub dux_context: DuxContext,
}

impl HostWorkFlow {
    pub fn new() -> HostWorkFlow {
        HostWorkFlow {
            task_flows: Vec::new(),
            final_status: HostWorkFlowStatus::NotRunYet,
            dux_context: DuxContext::new(),
        }
    }

    pub fn from(task_list: TaskList, dux_context: DuxContext) -> HostWorkFlow {
        let mut task_flows: Vec<TaskFlow> = Vec::new();

        for task_block in task_list.tasks.iter() {
            task_flows.push(TaskFlow::from(task_block.clone()))
        }

        HostWorkFlow {
            task_flows,
            final_status: HostWorkFlowStatus::NotRunYet,
            dux_context,
        }
    }

    pub fn dry_run(&mut self, hosthandler: &mut HostHandler) -> Result<(), Error> {
        let mut changes_required = false;

        for task_flow in self.task_flows.iter_mut() {
            match task_flow.dry_run(hosthandler, &mut self.dux_context) {
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
            match task_flow.apply(hosthandler, &mut self.dux_context) {
                Ok(()) => {}
                Err(error) => {
                    return Err(error);
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HostWorkFlowStatus {
    NotRunYet,
    AlreadyMatched,
    ChangeRequired,
    ApplySuccesful,
    ApplyWithAllowedFailure,
    ApplyFailed,
}

/// Withholds variables, either defined in advance by the user in HostList and/or Tasklist or defined at runtime (output of a Step saved as a variable). This struct is accessible by each step during the tasklist traversal.
#[derive(Clone, Debug)]
pub struct DuxContext {
    pub vars: HashMap<String, String>,
    pub tera_context: Context
}

impl DuxContext {
    pub fn new() -> DuxContext {
        DuxContext {
            vars: HashMap::new(),
            tera_context: Context::new()
        }
    }

    pub fn from(vars: Option<HashMap<String, String>>) -> DuxContext {
        match vars {
            Some(variables) => DuxContext {
                vars: variables.clone(),
                tera_context: Context::from_serialize(variables).unwrap()
            },
            None => DuxContext::new(),
        }
    }
}
