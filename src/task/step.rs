use serde::{Deserialize, Deserializer, Serialize};

use crate::modules::prelude::*;
use crate::task::moduleblock::ModuleBlockExpectedState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Step {
    pub name: Option<String>,
    pub run_as: Option<String>,
    pub with_sudo: Option<bool>,
    pub allowed_to_fail: Option<bool>,
    // pub prelogic -> TODO
    // pub postlogic -> TODO

    // This attribute is filled by the .parsemodule() method based on the rest of
    // the attributes (one per module). After applying this method, the moduleblock
    // attribute holds the Expected State ready to be used by the rest of the workflow.
    pub moduleblock: Option<ModuleBlockExpectedState>,

    // FIXME: Having an attribute per module is at the moment the only way found to be able to write "apt:" and not "!apt".
    // It requires parsemodule() method to check that only one attribute per step is filled.
    // **BEACON_1**
    pub lineinfile: Option<LineInFileBlockExpectedState>,
    pub command: Option<CommandBlockExpectedState>,
    pub apt: Option<AptBlockExpectedState>,
    pub dnf: Option<YumDnfBlockExpectedState>,
    #[serde(default, deserialize_with = "deserialize_argumentlessmodule")]
    pub ping: Option<Option<PingBlockExpectedState>>, // Double wrapping in order to have Serde distinguish between missing field and None value
    pub yum: Option<YumDnfBlockExpectedState>,
}

impl Step {
    pub fn parsemodule(&mut self) -> Result<(), String> {
        let mut counter: u32 = 0; // Used to check that only one module is used per Step

        // **BEACON_2**
        if let Some(content) = self.lineinfile.clone() {
            counter += 1;
            self.moduleblock = Some(ModuleBlockExpectedState::LineInFile(content));
        }
        if let Some(content) = self.command.clone() {
            counter += 1;
            self.moduleblock = Some(ModuleBlockExpectedState::Command(content));
        }
        if let Some(content) = self.apt.clone() {
            counter += 1;
            self.moduleblock = Some(ModuleBlockExpectedState::Apt(content));
        }
        if let Some(content) = self.dnf.clone() {
            counter += 1;
            self.moduleblock = Some(ModuleBlockExpectedState::Dnf(content));
        }
        if let Some(_content) = self.ping.clone() {
            counter += 1;
            self.moduleblock = Some(ModuleBlockExpectedState::Ping(PingBlockExpectedState {}));
        } // Ping "content" is always null at the moment
        if let Some(content) = self.yum.clone() {
            counter += 1;
            self.moduleblock = Some(ModuleBlockExpectedState::Yum(content));
        }

        if counter > 1 {
            return Err(String::from("Too much modules defined in this step"));
        } else {
            return Ok(());
        }
    }
}

// Any value that is present is considered Some value, including null. This way, we can use
// argument-less modules like Ping by writing "ping:" and Serde won't confuse it with a missing field.
fn deserialize_argumentlessmodule<'a, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: Deserialize<'a>,
    D: Deserializer<'a>,
{
    Deserialize::deserialize(deserializer).map(Some)
}
