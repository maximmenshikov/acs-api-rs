use serde::{Deserialize, Serialize};
use crate::util::accessor::*;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[repr(C)]
pub struct AddDeleteObject {
    #[serde(default = "unset_str")]
    pub name: String,
    #[serde(default = "unset_str")]
    pub object_name: String,
}

impl AddDeleteObject {
    pub fn new(add: bool, object_name: &str) -> Self {
        return AddDeleteObject {
            name: (if add { "addObject" } else { "deleteObject" }).to_string(),
            object_name: object_name.to_string(),
        };
    }
}
