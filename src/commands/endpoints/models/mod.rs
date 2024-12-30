pub(crate) mod list;

use serde::Serialize;
use serde_repr::Deserialize_repr;

#[derive(Debug, Serialize, Deserialize_repr, PartialEq)]
#[repr(u32)]
pub(crate) enum EndpointStatus {
    Up = 1,
    Down = 2,
    #[serde(other)]
    Unknown = 0,
}

#[derive(Debug, Serialize, Deserialize_repr, PartialEq)]
#[repr(u32)]
pub(crate) enum EndpointType {
    Docker = 1,
    Agent = 2,
    Azure = 3,
    #[serde(other)]
    Unknown = 0,
}
