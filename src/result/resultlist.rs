use crate::result::taskresult::TaskResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultList {
    pub taskresults: Vec<TaskResult>,
}

impl ResultList {
    pub fn new() -> ResultList {
        ResultList {
            taskresults: Vec::new(),
        }
    }

    // The 'results' field could be turned into an Option but this complexifies the apply_changelist() method
    // in change.rs (we need to deconstruct...etc). For now, results = 'None' is just an empty vector.
    // TODO : turn 'results' into an Option<Vec<TaskResult>>.
    pub fn none() -> ResultList {
        // TODO : set all blockmatrix results to None as well
        ResultList {
            taskresults: Vec::new(),
        }
    }

    pub fn from(taskresults: Vec<TaskResult>) -> ResultList {
        ResultList { taskresults }
    }
}
