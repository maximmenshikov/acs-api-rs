use crate::util::accessor::*;
use serde::{Deserialize, Serialize};

fn unset_vec_vec_str() -> Vec<Vec<String>> {
    Vec::new()
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[repr(C)]
pub struct AcsDeviceId {
    #[serde(default = "unset_str", rename = "_Manufacturer")]
    pub manufacturer: String,
    #[serde(default = "unset_str", rename = "_OUI")]
    pub oui: String,
    #[serde(default = "unset_str", rename = "_ProductClass")]
    pub product_class: String,
    #[serde(default = "unset_str", rename = "_SerialNumber")]
    pub serial_number: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[repr(C)]
pub struct AcsTask {
    #[serde(default = "unset_str", rename = "_id")]
    pub id: String,

    #[serde(default = "unset_str")]
    pub device: String,

    #[serde(default = "unset_str")]
    pub name: String,

    #[serde(default = "unset_str")]
    pub timestamp: String,

    /// Populated for addObject / deleteObject tasks
    #[serde(default = "unset_str", rename = "objectName")]
    pub object_name: String,

    /// Populated for setParameterValues tasks: [[param, value], ...]
    #[serde(default = "unset_vec_vec_str", rename = "parameterValues")]
    pub parameter_values: Vec<Vec<String>>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[repr(C)]
pub struct AcsDevice {
    #[serde(default = "unset_str", rename = "_id")]
    pub id: String,
    #[serde(default = "unset_acs_device_id", rename = "_deviceId")]
    pub device_id: AcsDeviceId,

    #[serde(default = "unset_str", rename = "_lastInform")]
    pub last_inform: String,

    #[serde(default = "unset_str", rename = "_registered")]
    pub registered: String,
}
