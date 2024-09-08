use crate::error::Error;
use crate::task::step::Step;
use crate::change::stepchange::StepChange;
use crate::result::stepresult::StepResult;
use crate::connection::hosthandler::HostHandler;
use crate::connection::specification::Privilege;
use crate::result::apicallresult::ApiCallStatus;

#[derive(Debug, Clone)]
pub struct StepFlow {
    pub step_expected: Step,
    pub allowed_to_fail: bool,
    pub step_change: Option<StepChange>,
    pub step_result: Option<StepResult>,
    pub step_status: StepStatus,
}

impl StepFlow {
    pub fn from(step: Step) -> StepFlow {
        StepFlow {
            step_expected: step.clone(),
            allowed_to_fail: match step.allowed_to_fail {
                Some(value) => value,
                None => false,
            },
            step_change: None,
            step_result: None,
            step_status: StepStatus::NotRunYet,
        }
    }

    pub fn dry_run(&mut self, hosthandler: &mut HostHandler) -> Result<(), Error> {
        let privilege = match self.step_expected.with_sudo {
            None => match &self.step_expected.run_as {
                None => Privilege::Usual,
                Some(username) => Privilege::AsUser(username.into()),
            },
            Some(value) => {
                if value {
                    Privilege::WithSudo
                } else {
                    match &self.step_expected.run_as {
                        None => Privilege::Usual,
                        Some(username) => Privilege::AsUser(username.into()),
                    }
                }
            }
        };

        match self
            .step_expected
            .moduleblock
            .dry_run_moduleblock(hosthandler, privilege)
        {
            Ok(mbchange) => {
                match &mbchange {
                    StepChange::AlreadyMatched(_) => {
                        self.step_status = StepStatus::AlreadyMatched;
                    }
                    StepChange::ModuleApiCalls(_) => {
                        self.step_status = StepStatus::ChangeRequired;
                    }
                }
                self.step_change = Some(mbchange);
            }
            Err(error) => {
                return Err(error);
            }
        }

        Ok(())
    }
    pub fn apply(&mut self, hosthandler: &mut HostHandler) -> Result<(), Error> {
        // Check that dry_run performed first
        match self.step_status {
            StepStatus::NotRunYet => {} // Error
            StepStatus::ApplySuccessful
            | StepStatus::ApplyFailedButAllowed
            | StepStatus::ApplyFailed => {} // Error
            StepStatus::AlreadyMatched => {
                return Ok(());
            }
            StepStatus::ChangeRequired => {
                match &self.step_change {
                    Some(change) => {
                        let result = change.apply_moduleblockchange(hosthandler);
                        let mut step_status = StepStatus::ApplySuccessful;

                        for apicallresult in result.apicallresults.clone().iter() {
                            match apicallresult.status {
                                ApiCallStatus::Failure(_) => {
                                    if self.allowed_to_fail {
                                        step_status = StepStatus::ApplyFailedButAllowed;
                                    } else {
                                        step_status = StepStatus::ApplyFailed;
                                        break;
                                    }
                                }
                                _ => {}
                            }
                        }
                        self.step_status = step_status;
                        self.step_result = Some(result);
                    }
                    None => {} // Error
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum StepStatus {
    NotRunYet,
    AlreadyMatched,
    ChangeRequired,
    ApplySuccessful,
    ApplyFailedButAllowed,
    ApplyFailed,
}
