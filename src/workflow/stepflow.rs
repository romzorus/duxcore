use crate::error::Error;
use crate::task::step::Step;
use crate::change::stepchange::StepChange;
use crate::result::stepresult::StepResult;
use crate::connection::hosthandler::HostHandler;
use crate::connection::specification::Privilege;
use crate::result::apicallresult::ApiCallStatus;
use crate::workflow::hostworkflow::DuxContext;


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

    pub fn dry_run(&mut self, hosthandler: &mut HostHandler, dux_context: &mut DuxContext) -> Result<(), Error> {
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
    pub fn apply(&mut self, hosthandler: &mut HostHandler, dux_context: &mut DuxContext) -> Result<(), Error> {
        // Check that dry_run performed first
        // match self.step_status {
        //     StepStatus::NotRunYet => {
        //         return Err(Error::WorkFlowNotFollowed(
        //             "Trying to apply a change list but dry_run not run yet. Please run dry_run step before trying to apply.".into()
        //         ))
        //     }
        //     StepStatus::ApplySuccessful
        //     | StepStatus::ApplyFailedButAllowed
        //     | StepStatus::ApplyFailed => {
        //         return Err(Error::WorkFlowNotFollowed(
        //             "Trying to apply an already-applied step".into()
        //         ))
        //     }
        //     StepStatus::AlreadyMatched => {
        //         return Ok(());
        //     }
        //     StepStatus::ChangeRequired => {
        //         match &self.step_change {
        //             Some(change) => {
        //                 let result = change.apply_moduleblockchange(hosthandler);
        //                 let mut step_status = StepStatus::ApplySuccessful;

        //                 for apicallresult in result.apicallresults.clone().iter() {
        //                     match apicallresult.status {
        //                         ApiCallStatus::Failure(_) => {
        //                             if self.allowed_to_fail {
        //                                 step_status = StepStatus::ApplyFailedButAllowed;
        //                             } else {
        //                                 step_status = StepStatus::ApplyFailed;
        //                                 break;
        //                             }
        //                         }
        //                         _ => {}
        //                     }
        //                 }

        //                 // Register : push step result to context under the specified variable name
        //                 if let Some(variable_name) = &self.step_expected.register {
        //                     dux_context.tera_context.insert(variable_name, &result);
        //                 }

        //                 self.step_status = step_status;
        //                 self.step_result = Some(result);
        //             }
        //             None => {
        //                 return Err(Error::WorkFlowNotFollowed(
        //                     "StepStatus = ChangeRequired but StepChange is empty. Something needs to be done but no information on what to do is provided.".into()
        //                 ))
        //             }
        //         }
        //     }
        // }

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
            .consider_context(dux_context).unwrap()
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

                // Register : push step result to context under the specified variable name
                if let Some(variable_name) = &self.step_expected.register {
                    dux_context.tera_context.insert(variable_name, &result);
                }

                self.step_status = step_status;
                self.step_result = Some(result);
            }
            None => {
                return Err(Error::WorkFlowNotFollowed(
                    "StepStatus = ChangeRequired but StepChange is empty. Something needs to be done but no information on what to do is provided.".into()
                ))
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
