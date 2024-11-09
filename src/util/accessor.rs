use crate::device::AcsDeviceId;
use crate::parameter_value::*;

pub fn unset_str() -> String {
    return "".to_string();
}

pub fn unset_vec_str() -> Vec<String> {
    return Vec::new();
}

pub fn unset_parameter_value() -> ParameterValue {
    return ParameterValue::new("", "", "");
}

pub fn unset_vec_parameter_value() -> Vec<ParameterValue> {
    return Vec::new();
}

pub fn unset_acs_device_id() -> AcsDeviceId {
    return AcsDeviceId { manufacturer: "".to_string(), oui: "".to_string(), product_class: "".to_string(),
        serial_number: "".to_string()
    };
}