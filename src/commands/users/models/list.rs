use serde::{Deserialize, Serialize};
use serde_repr::Deserialize_repr;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct User {
    id: u32,
    username: String,
    role: UserRole,
}

#[derive(Debug, Serialize, Deserialize_repr, PartialEq)]
#[repr(u32)]
pub(crate) enum UserRole {
    Administrator = 1,
    Regular = 2,
    #[serde(other)]
    Unknown = 0,
}
