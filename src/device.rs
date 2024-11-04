use serde::{Deserialize, Serialize};
use crate::util::accessor::*;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[repr(C)]
pub struct AcsDevice {
    #[serde(default = "unset_str")]
    pub id: String,
    #[serde(default = "unset_str")]
    pub serial_number: String,
    #[serde(default = "unset_str")]
    pub manufacturer: String,
    #[serde(default = "unset_str")]
    pub model: String,
    #[serde(default = "unset_str")]
    pub firmware_version: String,
    #[serde(default = "unset_str")]
    pub last_online: String,
    #[serde(default = "unset_str")]
    pub created_at: String,
    #[serde(default = "unset_str")]
    pub updated_at: String,
    #[serde(default = "unset_str")]
    pub status: String,
}
