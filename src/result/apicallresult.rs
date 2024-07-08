use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiCallResult {
    pub exitcode: Option<i32>,
    pub output: Option<String>,
    pub status: ApiCallStatus,
}

impl ApiCallResult {
    pub fn new() -> ApiCallResult {
        ApiCallResult {
            exitcode: None,
            output: None,
            status: ApiCallStatus::Unset,
        }
    }

    pub fn none() -> ApiCallResult {
        ApiCallResult {
            exitcode: None,
            output: None,
            status: ApiCallStatus::None,
        }
    }

    pub fn from(
        exitcode: Option<i32>,
        output: Option<String>,
        status: ApiCallStatus,
    ) -> ApiCallResult {
        ApiCallResult {
            exitcode,
            output,
            status,
        }
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
