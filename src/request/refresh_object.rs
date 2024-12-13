use crate::util::accessor::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[repr(C)]
pub struct RefreshObject {
    #[serde(default = "unset_str")]
    pub name: String,
    #[serde(default = "unset_str")]
    pub object_name: String,
}

impl RefreshObject {
    pub fn new(object_name: &str) -> Self {
        return RefreshObject {
            name: "refreshObject".to_string(),
            object_name: object_name.to_string(),
        };
    }
}
