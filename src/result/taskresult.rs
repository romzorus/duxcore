use crate::result::stepresult::StepResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    pub stepresults: Option<Vec<StepResult>>,
}

impl TaskResult {
    pub fn new() -> TaskResult {
        TaskResult {
            stepresults: Some(Vec::new()),
        }
    }

    pub fn none() -> TaskResult {
        TaskResult { stepresults: None }
    }

    pub fn from(stepresults: Option<Vec<StepResult>>) -> TaskResult {
        TaskResult { stepresults }
    }
}
