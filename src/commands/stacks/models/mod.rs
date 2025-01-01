pub(crate) mod deploy;
pub(crate) mod list;
pub(crate) mod resource_control;

use serde::Serialize;
use serde_repr::Deserialize_repr;

#[derive(Debug, Serialize, Deserialize_repr, PartialEq)]
#[repr(u32)]
pub(crate) enum StackStatus {
    Active = 1,
    Inactive = 2,
    #[serde(other)]
    Unknown = 0,
}

#[derive(Debug, Serialize, Deserialize_repr, PartialEq)]
#[repr(u32)]
pub(crate) enum StackType {
    Swarm = 1,
    Compose = 2,
    #[serde(other)]
    Unknown = 0,
}
