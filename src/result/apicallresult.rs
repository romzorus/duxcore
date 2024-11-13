use serde::{Deserialize, Serialize};

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

    pub fn from(rc: Option<i32>, raw_output: Option<String>, status: ApiCallStatus) -> ApiCallResult {

        let output = match raw_output {
            Some(raw_output_content) => {
                Some(
                    // raw_output_content
                    //     .chars()
                    //     .map(|x| if x.is_control() { ' ' } else { x })
                    //     .collect()
                    raw_output_content
                        .chars()
                        .filter(|c| c.is_ascii() && ! c.is_control())
                        .collect()
                )
            }
            None => None
        };
        
        ApiCallResult { rc, output, status }
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
