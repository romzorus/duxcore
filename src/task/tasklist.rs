use crate::change::changelist::ChangeList;
use crate::change::taskchange::TaskChange;
use crate::connection::hosthandler::HostHandler;
use crate::host::hosts::Host;
use crate::error::Error;
use crate::task::taskblock::TaskBlock;
use crate::task::contentformat::json::json_tasklist_parser;
use crate::task::contentformat::yaml::yaml_tasklist_parser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskList {
    pub tasks: Vec<TaskBlock>,
}

impl TaskList {
    pub fn new() -> TaskList {
        TaskList {
            tasks: Vec::<TaskBlock>::new(),
        }
    }
    pub fn from(tasks: Vec<TaskBlock>) -> TaskList {
        TaskList { tasks }
    }
    pub fn from_str(raw_content: &str, content_type: TaskListFileType, host: &Host) -> Result<TaskList, Error> {
        match content_type {
            TaskListFileType::Yaml => {
                match yaml_tasklist_parser(raw_content, host) {
                    Ok(mut parsed_content) => {
                        for (taskindex, taskcontent) in parsed_content.clone().tasks.iter().enumerate() {
                            for (stepindex, _stepcontent) in taskcontent.steps.iter().enumerate() {
                                parsed_content.tasks[taskindex].steps[stepindex].parsemodule().unwrap();
                                // TODO : error handling required here
                            }
                        }
                        return Ok(parsed_content);
                    }
                    Err(error) => {
                        return Err(Error::FailedInitialization(
                            format!("{:?}", error)
                        ));
                    }
                }
            }
            TaskListFileType::Json => {
                match json_tasklist_parser(raw_content, host) {
                    Ok(mut parsed_content) => {
                        for (taskindex, taskcontent) in parsed_content.clone().tasks.iter().enumerate()
                        {
                            for (stepindex, _stepcontent) in taskcontent.steps.iter().enumerate() {
                                parsed_content.tasks[taskindex].steps[stepindex].parsemodule().unwrap();
                                // TODO : error handling required here
                            }
                        }
                        return Ok(parsed_content);
                    }
                    Err(error) => {
                        return Err(Error::FailedInitialization(
                            format!("{:?}", error)
                        ));
                    }
                }
            }
            TaskListFileType::Unknown => {
                // Unknown format -> Try YAML -> Try JSON -> Failed
                match yaml_tasklist_parser(raw_content, host) {
                    Ok(mut parsed_content) => {
                        for (taskindex, taskcontent) in parsed_content.clone().tasks.iter().enumerate() {
                            for (stepindex, _stepcontent) in taskcontent.steps.iter().enumerate() {
                                parsed_content.tasks[taskindex].steps[stepindex].parsemodule().unwrap();
                                // TODO : error handling required here
                            }
                        }
                        return Ok(parsed_content);
                    }
                    Err(yaml_try_error) => {
                        match json_tasklist_parser(raw_content, host) {
                            Ok(mut parsed_content) => {
                                for (taskindex, taskcontent) in parsed_content.clone().tasks.iter().enumerate()
                                {
                                    for (stepindex, _stepcontent) in taskcontent.steps.iter().enumerate() {
                                        parsed_content.tasks[taskindex].steps[stepindex].parsemodule().unwrap();
                                        // TODO : error handling required here
                                    }
                                }
                                return Ok(parsed_content);
                            }
                            Err(json_try_error) => {
                                return Err(Error::FailedInitialization(
                                    format!("Unable to parse file. YAML : {:?}, JSON : {:?}", yaml_try_error, json_try_error)
                                ));
                            }
                        }
                    }
                }
            }
        }
    }
    pub fn from_file(file_path: &str, file_type: TaskListFileType, host: &Host) -> Result<TaskList, Error> {
        match std::fs::read_to_string(file_path) {
            Ok(file_content) => {
                return TaskList::from_str(&file_content, file_type, host);
            }
            Err(error) => {
                return Err(Error::FailedInitialization(
                    format!("{} : {}", file_path, error)
                ));
            }
        }
    }
    pub fn dry_run_tasklist(
        &self,
        _correlationid: String,
        hosthandler: &mut HostHandler,
    ) -> Result<ChangeList, Error> {
        let mut list: Vec<TaskChange> = Vec::new();

        for taskcontent in self.tasks.clone().iter() {
            match taskcontent.dry_run_task(hosthandler) {
                Ok(taskchange) => {
                    list.push(taskchange);
                }
                Err(e) => return Err(e),
            }
        }
        return Ok(ChangeList::from(Some(list), hosthandler.clone()));
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum RunningMode {
    DryRun, // Only check what needs to be done to match the expected situation
    Apply,  // Actually apply the changes required to match the expected situation
}

pub enum TaskListFileType {
    Yaml,
    Json,
    Unknown
}