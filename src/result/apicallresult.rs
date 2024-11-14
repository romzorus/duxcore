use serde::{Deserialize, Serialize};
use regex::Regex;

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
                let re = Regex::new(r"[\u{0000}-\u{001F}\u{007F}-\u{009F}]").unwrap();
                
                // let new_output = raw_output_content
                    // .chars()
                    // .map(|x| if x.is_ascii_control() || x.is_control() { ' ' } else { x })
                    // .collect();
                let new_output = re.replace_all(raw_output_content.as_str(), "").to_string();

                Some(new_output)
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
