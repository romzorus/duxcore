use serde::{Deserialize, Serialize};
use unicode_categories::UnicodeCategories;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiCallResult {
    pub rc: Option<i32>,
    pub output: Option<String>,
    pub status: ApiCallStatus,
}

impl ApiCallResult {
    pub fn new() -> ApiCallResult {
        ApiCallResult {
            rc: None,
            output: None,
            status: ApiCallStatus::Unset,
        }
    }

    pub fn none() -> ApiCallResult {
        ApiCallResult {
            rc: None,
            output: None,
            status: ApiCallStatus::None,
        }
    }

    pub fn from(rc: Option<i32>, output: Option<String>, status: ApiCallStatus) -> ApiCallResult {
       
        let new_output = match output {
            Some(raw_output) => {
                let parsed_raw_output = raw_output
                    .chars()
                    .filter(|&c| !c.is_control() && !c.is_other_control())
                    .collect::<String>();

                Some(parsed_raw_output.trim().into())
            }
            None => None
        };

        ApiCallResult { rc, output: new_output, status }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApiCallStatus {
    Unset,
    None,
    ChangeSuccessful(String),
    Failure(String),
    AllowedFailure(String),
}
