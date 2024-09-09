use crate::change::stepchange::StepChange;
use crate::connection::hosthandler::HostHandler;
use crate::connection::specification::Privilege;
use crate::error::Error;
use crate::result::apicallresult::{ApiCallResult, ApiCallStatus};
use crate::task::moduleblock::ModuleApiCall;
use crate::task::moduleblock::{Apply, DryRun};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DebugBlockExpectedState {
    msg: String,
    // var: Option<String>, // To be done
}

impl DryRun for DebugBlockExpectedState {
    fn dry_run_block(
        &self,
        hosthandler: &mut HostHandler,
        privilege: Privilege,
    ) -> Result<StepChange, Error> {
        
        return Ok(StepChange::matched(self.msg.as_str()));
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DebugApiCall {}

impl Apply for DebugApiCall {
    fn display(&self) -> String {
        "Debug module".into()
    }

    fn apply_moduleblock_change(&self, hosthandler: &mut HostHandler) -> ApiCallResult {
        return ApiCallResult::none();
    }
}
