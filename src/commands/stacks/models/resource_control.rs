use serde::{Deserialize, Serialize};

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
