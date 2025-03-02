use crate::util::accessor::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[repr(C)]
pub struct DownloadCommand {
    #[serde(default = "unset_str")]
    pub name: String,
    #[serde(default = "unset_str")]
    pub file: String,
}

impl DownloadCommand {
    pub fn new(file: &str) -> Self {
        return DownloadCommand {
            name: "download".to_string(),
            file: file.to_string(),
        };
    }
}
