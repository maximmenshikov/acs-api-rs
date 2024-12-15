use serde_json::Value;
use serde::Serializer;
use crate::util::accessor::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, PartialEq, Clone, Debug)]
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
            value_type: value_type.to_string(),
        };
    }

    pub fn to_value(&self) -> Vec<String> {
        return vec![
            self.parameter.clone(),
            self.value.clone(),
            self.value_type.clone(),
        ];
    }
}

impl Serialize for ParameterValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.value_type.as_str() {
            "xsd:string" => {
                let output = vec![
                    Value::String(self.parameter.clone()),
                    Value::String(self.value.clone())];
                output.serialize(serializer)
            }
            "xsd:boolean" => {
                let parsed_value = self.value.parse::<bool>().unwrap_or(false);
                let output = vec![
                    Value::String(self.parameter.clone()),
                    Value::Bool(parsed_value)];
                output.serialize(serializer)
            }
            "xsd:unsignedInt" => {
                let parsed_value = self.value.parse::<u32>().unwrap_or(0);
                let output = vec![
                    Value::String(self.parameter.clone()),
                    Value::Number(parsed_value.into())];
                output.serialize(serializer)
            }
            _ => {
                let output = vec![self.parameter.clone(), self.value.clone()];
                output.serialize(serializer)
            }
        }
    }
}
