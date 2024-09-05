use crate::change::stepchange::StepChange;
use crate::change::taskchange::TaskChange;
use crate::connection::hosthandler::HostHandler;
use crate::connection::specification::Privilege;
use crate::error::Error;
use crate::task::step::{ParsingStep, Step};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskBlock {
    pub name: Option<String>,
    pub steps: Vec<Step>,
    pub with_sudo: Option<bool>,
}

impl TaskBlock {
    pub fn new() -> TaskBlock {
        TaskBlock {
            name: None,
            steps: Vec::new(),
            with_sudo: None,
        }
    }

    pub fn from(name: Option<String>, steps: Vec<Step>, with_sudo: Option<bool>) -> TaskBlock {
        TaskBlock {
            name,
            steps,
            with_sudo,
        }
    }

    pub fn dry_run_task(&self, hosthandler: &mut HostHandler) -> Result<TaskChange, Error> {
        let mut mbchangeslist: Vec<StepChange> = Vec::new();
        let mut allowed_failures: Vec<bool> = Vec::new();

        // TODO : add some checking (with_sudo and run_as need to be mutually exclusive)
        for step in self.clone().steps.into_iter() {
            let privilege = match step.with_sudo {
                None => match step.run_as {
                    None => Privilege::Usual,
                    Some(username) => Privilege::AsUser(username),
                },
                Some(value) => {
                    if value {
                        Privilege::WithSudo
                    } else {
                        match step.run_as {
                            None => Privilege::Usual,
                            Some(username) => Privilege::AsUser(username),
                        }
                    }
                }
            };

            match step.moduleblock.dry_run_moduleblock(
                hosthandler,
                privilege,
                step.allowed_to_fail.unwrap_or(false),
            ) {
                Ok((moduleblockchange, allowed_to_fail)) => {
                    mbchangeslist.push(moduleblockchange);
                    allowed_failures.push(allowed_to_fail);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        return Ok(TaskChange::from(mbchangeslist, allowed_failures));
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsingTaskBlock {
    pub name: Option<String>,
    pub steps: Vec<ParsingStep>,
    pub with_sudo: Option<bool>,
}

impl ParsingTaskBlock {
    pub fn parse_task_block(&self) -> Result<TaskBlock, Error> {
        let mut steps: Vec<Step> = Vec::new();
        for parsing_step in self.steps.iter() {
            match parsing_step.parsemodule() {
                Ok(step) => {
                    steps.push(step);
                }
                Err(error) => {
                    return Err(error);
                }
            }
        }

        Ok(TaskBlock {
            name: self.name.clone(),
            steps: steps,
            with_sudo: self.with_sudo.clone(),
        })
    }
}
