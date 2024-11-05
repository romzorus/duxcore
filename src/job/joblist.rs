use crate::job::job::Job;
use crate::output::joblist_output::JobListOutput;
use crate::host::hostlist::HostList;
use crate::workflow::hostworkflow::DuxContext;
use crate::connection::host_connection::HostConnectionInfo;
use crate::error::Error;

/// A JobList is just a Vec of Jobs on which convenient methods are defined. It simplifies the handling of multiple hosts.
#[derive(Debug, Clone)]
pub struct JobList {
    pub job_list: Option<Vec<Job>>
}

impl JobList {
    pub fn new() -> JobList {
        JobList {
            job_list: Some(Vec::new())
        }
    }

    pub fn from_hostlist(host_list: HostList) -> JobList {
        match host_list.hosts {
            Some(host_list_content) => {
                let mut jobs: Vec<Job> = Vec::new();

                for host in host_list_content {
                    let mut job = Job::new();
                    job.set_address(&host.address).unwrap();
                    job.set_context(DuxContext::from(host));

                    jobs.push(job);

                }

                JobList { job_list: Some(jobs) }
                }
            None => {
                JobList { job_list: None }
            }
        }
    }

    pub fn add_job(&mut self, job: Job) {
        match &self.job_list {
            Some(_jobs) => {
                self.job_list.as_mut().unwrap().push(job);
            }
            None => {
                self.job_list = Some(vec![job]);
            }
        }
    }

    pub fn display(&mut self) -> String {
        let joblist_output = JobListOutput::from_joblist(self);
        serde_json::to_string(&joblist_output).unwrap()
    }

    pub fn display_pretty(&mut self) -> String {
        let joblist_output = JobListOutput::from_joblist(self);
        serde_json::to_string_pretty(&joblist_output).unwrap()
    }

    pub fn set_connection_for_all_jobs(
        &mut self,
        host_connection_info: HostConnectionInfo,
    ) -> Result<&mut Self, Error> {
        if let HostConnectionInfo::Unset = host_connection_info {
            Err(Error::WrongInitialization(format!(
                "No point in initializing connection info to HostConnectionInfo::Unset"
            )))
        } else {
            if let Some(jobs) = &mut self.job_list {
                for job in jobs {
                    job.host_connection_info = host_connection_info.clone();
                }
            }
            
            Ok(self)
        }
    }

    pub fn dry_run(&mut self) -> Result<(), Error> {
        if let Some(jobs) = &mut self.job_list {
            for job in jobs {
                job.dry_run()?;
            }
        }
        
        Ok(())
    }

    pub fn apply(&mut self) -> Result<(), Error> {
        if let Some(jobs) = &mut self.job_list {
            for job in jobs {
                job.apply()?;
            }
        }
        
        Ok(())
    }
}
