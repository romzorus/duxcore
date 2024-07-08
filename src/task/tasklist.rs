use crate::change::changelist::ChangeList;
use crate::change::taskchange::TaskChange;
use crate::connection::hosthandler::HostHandler;
use crate::error::Error;
use crate::task::taskblock::TaskBlock;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskList {
    pub tasks: Vec<TaskBlock>,
}

impl TaskList {
    pub fn new() -> TaskList {
        TaskList {
            tasks: Vec::<TaskBlock>::new(),
        }
    }
    pub fn from(tasks: Vec<TaskBlock>) -> TaskList {
        TaskList { tasks }
    }
    pub fn dry_run_tasklist(
        &self,
        _correlationid: String,
        hosthandler: &mut HostHandler,
    ) -> Result<ChangeList, Error> {
        let mut list: Vec<TaskChange> = Vec::new();

        for taskcontent in self.tasks.clone().iter() {
            match taskcontent.dry_run_task(hosthandler) {
                Ok(taskchange) => {
                    list.push(taskchange);
                }
                Err(e) => return Err(e),
            }
        }
        return Ok(ChangeList::from(Some(list), hosthandler.clone()));
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum RunningMode {
    DryRun, // Only check what needs to be done to match the expected situation
    Apply,  // Actually apply the changes required to match the expected situation
}
