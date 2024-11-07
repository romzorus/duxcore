use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;

use crate::job::job::Job;
use crate::output::joblist_output::JobListOutput;
use crate::host::hostlist::HostList;
use crate::workflow::hostworkflow::DuxContext;
use crate::connection::host_connection::HostConnectionInfo;
use crate::error::Error;
use crate::task::tasklist::TaskListFileType;
use crate::task::tasklist::TaskList;

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

    pub fn from_hostlist_as_str(raw_content: &str) -> Result<JobList, Error> {
        match HostList::from_str(raw_content) {
            Ok(host_list_content) => {
                Ok(JobList::from_hostlist(host_list_content))
            }
            Err(error) => Err(error)
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

    pub fn set_connection(
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

    pub fn set_context(&mut self, context: DuxContext) -> &mut Self {
        if let Some(jobs) = &mut self.job_list {
            for job in jobs {
                job.context = context.clone();
            }
        }
        
        self
    }

    pub fn set_var(&mut self, key: &str, value: &str) -> &mut Self {
        if let Some(jobs) = &mut self.job_list {
            for job in jobs {
                job.context.set_var(key, value);
            }
        }
        
        self
    }

    pub fn set_tasklist_from_str(&mut self, raw_content: &str, content_type: TaskListFileType) -> Result<&mut Self, Error>{
        if let Some(jobs) = &mut self.job_list {
            match TaskList::from_str(raw_content, content_type) {
                Ok(task_list) => {
                    for job in jobs {
                        job.tasklist = Some(task_list.clone());
                    }
                }
                Err(error) => {
                    return Err(error);
                },
            }
        }
        Ok(self)
    }

    pub fn set_tasklist_from_file(&mut self, file_path: &str, content_type: TaskListFileType) -> Result<&mut Self, Error>{
        if let Some(jobs) = &mut self.job_list {
            match TaskList::from_file(file_path, content_type) {
                Ok(task_list) => {
                    for job in jobs {
                        job.tasklist = Some(task_list.clone());
                    }
                }
                Err(error) => {
                    return Err(error);
                },
            }
        }
        Ok(self)
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
            jobs.par_iter_mut().for_each(|job| 
                job.apply().unwrap()
            );
        }
        
        Ok(())
    }
}