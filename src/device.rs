use serde::{Deserialize, Serialize};
use crate::util::accessor::*;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[repr(C)]
pub struct AcsDevice {
    #[serde(default = "unset_str")]
    pub id: String,
}
