use crate::result::apicallresult::ApiCallResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepResult {
    pub apicallresults: Vec<ApiCallResult>,
}

impl StepResult {
    pub fn new() -> StepResult {
        StepResult {
            apicallresults: Vec::new(),
        }
    }

    pub fn none() -> StepResult {
        StepResult {
            apicallresults: Vec::from([ApiCallResult::none()]),
        }
    }

    pub fn from(apicallresults: Vec<ApiCallResult>) -> StepResult {
        StepResult { apicallresults }
    }
}
