use serde::{Deserialize, Serialize};
use crate::util::accessor::*;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[repr(C)]
pub struct GetParameterValues {
    #[serde(default = "unset_str")]
    pub name: String,
    #[serde(default = "unset_vec_str")]
    pub parameter_names: Vec<String>,
}

impl GetParameterValues {
    pub fn new(parameter_names: Vec<String>) -> Self {
        return GetParameterValues {
            name: "getParameterNames".to_string(),
            parameter_names: parameter_names.clone(),
        };
    }
}
