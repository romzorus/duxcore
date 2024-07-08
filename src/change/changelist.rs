use crate::change::taskchange::TaskChange;
use crate::connection::hosthandler::HostHandler;
use crate::result::resultlist::ResultList;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeList {
    pub taskchanges: Option<Vec<TaskChange>>,
    // hosthandler: HostHandler,
}

impl ChangeList {
    pub fn new() -> ChangeList {
        ChangeList {
            taskchanges: Some(Vec::new()),
            // hosthandler: HostHandler::new(),
        }
    }

    pub fn from(taskchanges: Option<Vec<TaskChange>>, _hosthandler: HostHandler) -> ChangeList {
        ChangeList {
            taskchanges,
            // hosthandler,
        }
    }

    pub fn apply_changelist(&mut self, hosthandler: &mut HostHandler) -> ResultList {
        match &self.taskchanges {
            None => {
                return ResultList::none();
            }
            Some(taskchangelist) => {
                let mut tasklistresult = ResultList::new();

                for taskchange in taskchangelist.iter() {
                    tasklistresult
                        .taskresults
                        .push(taskchange.apply_taskchange(hosthandler));
                }

                return tasklistresult;
            }
        }
    }
}
