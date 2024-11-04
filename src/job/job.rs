use crate::connection::host_connection::HostConnectionInfo;
pub use crate::connection::hosthandler::HostHandler;
use crate::connection::hosthandler::HostHandlingInfo;
use crate::connection::specification::ConnectionMode;
use crate::error::Error;
use crate::host::hostlist::HostList;
use crate::output::job_output::JobOutput;
use crate::task::tasklist::TaskList;
use crate::task::tasklist::TaskListFileType;
use crate::workflow::hostworkflow::HostWorkFlowStatus;
use crate::workflow::hostworkflow::{DuxContext, HostWorkFlow};
use chrono::Utc;
use machineid_rs::{Encryption, HWIDComponent, IdBuilder};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct Job {
    pub address: HostAddress,
    pub host_connection_info: HostConnectionInfo,
    pub correlation_id: Option<String>,
    pub tasklist: Option<TaskList>,
    pub context: DuxContext,
    pub timestamp_start: Option<String>,
    pub timestamp_end: Option<String>,
    pub hostworkflow: Option<HostWorkFlow>,
    pub final_status: HostWorkFlowStatus,
}

impl Job {
    pub fn new() -> Job {
        Job {
            address: HostAddress::Unset,
            host_connection_info: HostConnectionInfo::Unset,
            correlation_id: None,
            tasklist: None,
            context: DuxContext::new(),
            timestamp_start: None,
            timestamp_end: None,
            hostworkflow: None,
            final_status: HostWorkFlowStatus::NotRunYet,
        }
    }

    pub fn get_address(&self) -> Result<String, Error> {
        match &self.address {
            HostAddress::LocalHost => Ok("localhost".to_string()),
            HostAddress::RemoteHost(address) => Ok(address.to_string()),
            HostAddress::Unset => Err(Error::MissingInitialization(format!("Unset address"))),
        }
    }

    /// Set host address
    pub fn set_address(&mut self, address: &str) -> Result<&mut Self, Error> {
        // TODO : Add controls on address content (invalid address with spaces or else...)
        match address.to_lowercase().as_str() {
            "localhost" => {
                self.address = HostAddress::LocalHost;
                Ok(self)
            }
            "127.0.0.1" => {
                self.address = HostAddress::LocalHost;
                Ok(self)
            }
            "" => {
                return Err(Error::WrongInitialization(format!("Empty address")));
            }
            _ => {
                self.address = HostAddress::RemoteHost(address.to_string());
                Ok(self)
            }
        }
    }

    /// Using a correlation id can be required in a distributed environment. If a machine is building Jobs and sending it to worker nodes, then the results will probably arrive in a random order, meaning it will hard to identify which results belong to which Job unless we use correlation ids.
    pub fn with_correlation_id(&mut self, with_correlation_id: bool) -> Result<&mut Self, Error> {
        if with_correlation_id {
            match IdBuilder::new(Encryption::MD5)
                .add_component(HWIDComponent::CPUID)
                .add_component(HWIDComponent::MacAddress)
                .add_component(HWIDComponent::MachineName)
                .add_component(HWIDComponent::Username)
                .build("dux")
            {
                Ok(salt) => {
                    let now = SystemTime::now();
                    let value = Sha256::digest(format!("{}{:?}", salt, now));
                    self.correlation_id = Some(format!("{:X}", value));
                    Ok(self)
                }
                Err(e) => {
                    return Err(Error::FailedInitialization(format!("{}", e)));
                }
            }
        } else {
            self.correlation_id = None;
            Ok(self)
        }
    }

    /// How do we connect to the target host ?
    pub fn set_connection(
        &mut self,
        host_connection_info: HostConnectionInfo,
    ) -> Result<&mut Self, Error> {
        if let HostConnectionInfo::Unset = host_connection_info {
            Err(Error::WrongInitialization(format!(
                "No point in initializing connection info to HostConnectionInfo::Unset"
            )))
        } else {
            self.host_connection_info = host_connection_info;
            Ok(self)
        }
    }

    // Define the task list
    pub fn set_tasklist_from_str(
        &mut self,
        raw_content: &str,
        content_type: TaskListFileType,
    ) -> Result<&mut Self, Error> {
        match TaskList::from_str(raw_content, content_type) {
            Ok(task_list) => {
                self.tasklist = Some(task_list);
                Ok(self)
            }
            Err(error) => Err(error),
        }
    }

    pub fn set_tasklist_from_file(
        &mut self,
        file_path: &str,
        content_type: TaskListFileType,
    ) -> Result<&mut Self, Error> {
        match TaskList::from_file(file_path, content_type) {
            Ok(task_list) => {
                self.tasklist = Some(task_list);
                Ok(self)
            }
            Err(error) => Err(error),
        }
    }

    pub fn set_context(&mut self, context: DuxContext) {
        self.context = context;
    }

    pub fn set_var(&mut self, key: &str, value: &str) {
        self.context.set_var(key, value);
    }

    /// "DRY_RUN" this job -> evaluate the difference between the expected state and the actual state of the given host
    pub fn dry_run(&mut self) -> Result<(), Error> {
        // Build a HostHandler
        let mut host_handler = match &self.address {
            HostAddress::Unset => {
                return Err(Error::MissingInitialization("address not set".into()));
            }
            HostAddress::LocalHost => {
                HostHandler::from("localhost".into(), self.host_connection_info.clone()).unwrap()
            }
            HostAddress::RemoteHost(host_address) => {
                HostHandler::from(host_address.into(), self.host_connection_info.clone()).unwrap()
            }
        };

        host_handler.init();

        self.timestamp_start = Some(format!("{}", Utc::now().format("%+").to_string()));

        match &mut self.hostworkflow {
            Some(host_work_flow) => {
                host_work_flow.dry_run(&mut host_handler)?;
                self.final_status = host_work_flow.final_status.clone();
            }
            None => {
                let mut host_work_flow =
                    HostWorkFlow::from(&self.tasklist.as_mut().unwrap(), self.context.clone());
                host_work_flow.dry_run(&mut host_handler)?;
                self.final_status = host_work_flow.final_status.clone();
                self.hostworkflow = Some(host_work_flow);
            }
        }

        self.timestamp_end = Some(format!("{}", Utc::now().format("%+").to_string()));

        Ok(())
    }

    /// "APPLY" this job -> evaluate what needs to be done to reach the expected state, then do it
    pub fn apply(&mut self) -> Result<(), Error> {
        // Build a HostHandler
        let mut host_handler = match &self.address {
            HostAddress::Unset => {
                return Err(Error::MissingInitialization("address not set".into()));
            }
            HostAddress::LocalHost => {
                HostHandler::from("localhost".into(), self.host_connection_info.clone()).unwrap()
            }
            HostAddress::RemoteHost(host_address) => {
                HostHandler::from(host_address.into(), self.host_connection_info.clone()).unwrap()
            }
        };

        host_handler.init();

        self.timestamp_start = Some(format!("{}", Utc::now().format("%+").to_string()));

        match &mut self.hostworkflow {
            Some(host_work_flow) => {
                host_work_flow.apply(&mut host_handler)?;
                self.final_status = host_work_flow.final_status.clone();
            }
            None => {
                let mut host_work_flow =
                    HostWorkFlow::from(&self.tasklist.as_mut().unwrap(), self.context.clone());
                host_work_flow.apply(&mut host_handler)?;
                self.final_status = host_work_flow.final_status.clone();
                self.hostworkflow = Some(host_work_flow);
            }
        }

        self.timestamp_end = Some(format!("{}", Utc::now().format("%+").to_string()));

        Ok(())
    }

    pub fn display(&self) -> String {
        let job_output = JobOutput::from_job(self);
        serde_json::to_string(&job_output).unwrap()
    }

    pub fn display_pretty(&self) -> String {
        let job_output = JobOutput::from_job(self);
        serde_json::to_string_pretty(&job_output).unwrap()
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum RunningMode {
    DryRun, // Only check what needs to be done to match the expected situation
    Apply,  // Actually apply the changes required to match the expected situation
    Unset,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum JobFinalStatus {
    Unset,
    AlreadyMatched,
    FailedDryRun(String),
    Changed,
    ChangedWithFailures,
    FailedChange,
    GenericFailed(String),
}

#[derive(Debug, Clone)]
pub enum HostAddress {
    Unset,
    LocalHost,
    RemoteHost(String), // IP/hostname
}
