use crate::error::Error;
use crate::host::hosts::Host;
use crate::task::taskblock::TaskBlock;
use crate::task::tasklist::TaskList;
use serde_json;
use tera::{Context, Tera};

pub fn json_tasklist_parser(tasklistcontent: &str, host: &Host) -> Result<TaskList, Error> {
    // Before turning TaskList content into Rust struct, let's parse the variables
    let mut tera = Tera::default();
    let context = match &host.vars {
        Some(host_vars_list) => Context::from_serialize(host_vars_list).unwrap(),
        None => Context::new(),
    };

    let tasklist_content_with_vars = tera.render_str(tasklistcontent, &context).unwrap();

    match serde_json::from_str::<Vec<TaskBlock>>(tasklist_content_with_vars.as_str()) {
        Ok(parsed_content) => {
            return Ok(TaskList::from(parsed_content));
        }
        Err(e) => return Err(Error::FailureToParseContent(format!("{:?}", e))),
    }
}
