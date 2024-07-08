use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct Host {
    pub address: String,
    pub vars: Option<HashMap<String, String>>,
    pub groups: Option<Vec<String>>,
}

impl Host {
    pub fn from_string(address: String) -> Host {
        Host {
            address,
            vars: None,
            groups: None,
        }
    }

    pub fn add_to_group(&mut self, groupname: &String) {
        match &self.groups {
            Some(group_list) => {
                let mut new_group_list = group_list.clone();
                new_group_list.push(groupname.clone());
                self.groups = Some(new_group_list);
            }
            None => {
                self.groups = Some(vec![groupname.clone()]);
            }
        }
    }

    pub fn add_vars(&mut self, newvars: &HashMap<String, String>) {
        match &self.vars {
            Some(oldvars) => {
                let mut new_vars_list = oldvars.clone();
                new_vars_list.extend(newvars.clone());
                self.vars = Some(new_vars_list);
            }
            None => {
                self.vars = Some(newvars.clone());
            }
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Group {
    pub name: String,
    pub vars: Option<HashMap<String, String>>,
    pub hosts: Option<Vec<String>>,
}
