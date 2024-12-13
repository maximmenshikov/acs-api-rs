use crate::parameter_value::ParameterValue;
use crate::util::accessor::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[repr(C)]
pub struct SetParameterValues {
    #[serde(default = "unset_str")]
    pub name: String,
    #[serde(default = "unset_vec_parameter_value")]
    pub parameter_values: Vec<ParameterValue>,
}

impl SetParameterValues {
    pub fn new(parameter_values: Vec<ParameterValue>) -> Self {
        return SetParameterValues {
            name: "setParameterValues".to_string(),
            parameter_values: parameter_values.clone(),
        };
    }
}
