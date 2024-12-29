use serde::{Deserialize, Serialize};
use crate::commands::stacks::models::{ResourceControl, StackStatus, StackType};
use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds::deserialize as from_ts;
use chrono::serde::ts_seconds_option::deserialize as from_ts_option;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub (crate) struct EnvVar {
    pub(crate) name: String,
    pub(crate) value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct StackDeploySwarmCreatePayload {
    pub(crate) env: Vec<EnvVar>,
    pub(crate) from_app_template: bool,
    pub(crate) name: String,
    pub(crate) stack_file_content: String,
    pub(crate) swarm_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct StackDeployUpdatePayload {
    pub(crate) env: Vec<EnvVar>,
    pub(crate) stack_file_content: String,
    pub(crate) pull_image: bool,
    pub(crate) prune: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct Stack {
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) env: Vec<EnvVar>,
    pub(crate) from_app_template: bool,
    pub(crate) swarm_id: String,
    pub(crate) r#type: StackType,
    pub(crate) status: StackStatus,
    pub(crate) endpoint_id: u32,
    #[serde(deserialize_with = "from_ts")]
    pub(crate) creation_date: DateTime<Utc>,
    pub(crate) created_by: String,
    #[serde(deserialize_with = "from_ts_option")]
    pub(crate) update_date: Option<DateTime<Utc>>,
    pub(crate) updated_by: Option<String>,
    pub(crate) resource_control: Option<ResourceControl>,
}
