pub(crate) mod deploy;
pub(crate) mod list;

use serde::{Deserialize, Serialize};
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct ResourceControl {
    id: u32,
    resource_id: String,
    r#type: u32,
    user_accesses: Vec<UserAccesses>,
    team_accesses: Vec<TeamAccesses>,
    public: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct UserAccesses {
    user_id: u32,
    access_level: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct TeamAccesses {
    team_id: u32,
    access_level: u32,
}
