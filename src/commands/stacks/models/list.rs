use crate::commands::stacks::models::{ResourceControl, StackStatus, StackType};
use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds::deserialize as from_ts;
use chrono::serde::ts_seconds_option::deserialize as from_ts_option;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct StackList {
    pub(crate) id: u32,
    pub(crate) name: String,
    r#type: StackType,
    status: StackStatus,
    swarm_id: String,
    endpoint_id: u32,
    #[serde(deserialize_with = "from_ts")]
    creation_date: DateTime<Utc>,
    created_by: String,
    #[serde(deserialize_with = "from_ts_option")]
    update_date: Option<DateTime<Utc>>,
    updated_by: Option<String>,
    resource_control: ResourceControl,
}
