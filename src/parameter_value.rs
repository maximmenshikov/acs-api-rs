use serde::{Deserialize, Serialize};
use crate::util::accessor::*;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[repr(C)]
pub struct ParameterValue {
    #[serde(default = "unset_str")]
    pub parameter: String,
    #[serde(default = "unset_str")]
    pub value: String,
    #[serde(default = "unset_str")]
    pub value_type: String,
}

impl ParameterValue {
    pub fn new(parameter: &str, value: &str, value_type: &str) -> Self {
        return ParameterValue {
            parameter: parameter.to_string(),
            value: value.to_string(),
            value_type: value_type.to_string()
        };
    }

    pub fn to_value(&self) -> Vec<String> {
        return vec! { self.parameter.clone(), self.value.clone(), self.value_type.clone() };
    }
}
