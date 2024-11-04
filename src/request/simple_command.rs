use serde::{Deserialize, Serialize};
use crate::util::accessor::*;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[repr(C)]
pub struct SimpleCommand {
    #[serde(default = "unset_str")]
    pub name: String,
}

impl SimpleCommand {
    pub fn new(command: &str) -> Self {
        return SimpleCommand {
            name: command.to_string(),
        };
    }
}
