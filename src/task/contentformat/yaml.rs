use crate::error::Error;
use crate::host::hosts::Host;
use crate::task::taskblock::{ParsingTaskBlock, TaskBlock};
use crate::task::tasklist::TaskList;
use serde_yaml;
use tera::{Context, Tera};

pub fn yaml_tasklist_parser(tasklistcontent: &str, host: &Host) -> Result<TaskList, Error> {
    // Before turning TaskList content into Rust struct, let's parse the variables
    let mut tera = Tera::default();
    let context = match &host.vars {
        Some(host_vars_list) => Context::from_serialize(host_vars_list).unwrap(),
        None => Context::new(),
    };

    let tasklist_content_with_vars = tera.render_str(tasklistcontent, &context).unwrap();

    match serde_yaml::from_str::<Vec<ParsingTaskBlock>>(tasklist_content_with_vars.as_str()) {
        Ok(parsed_content) => {
            let mut tasks: Vec<TaskBlock> = Vec::new();
            for parsed_task in parsed_content.iter() {
                match parsed_task.parse_task_block() {
                    Ok(task_block) => {
                        tasks.push(task_block);
                    }
                    Err(error) => {
                        return Err(error);
                    }
                }
            }
            return Ok(TaskList::from(tasks));
        }
        Err(e) => return Err(Error::FailureToParseContent(format!("{:?}", e))),
    }
}
