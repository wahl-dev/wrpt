use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct ResourceControl {
    pub(crate) id: u32,
    pub(crate) resource_id: String,
    pub(crate) r#type: u32,
    pub(crate) user_accesses: Vec<UserAccesses>,
    pub(crate) team_accesses: Vec<TeamAccesses>,
    pub(crate) public: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct UserAccesses {
    pub(crate) user_id: u32,
    pub(crate) access_level: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct TeamAccesses {
    pub(crate) team_id: u32,
    pub(crate) access_level: u32,
}
