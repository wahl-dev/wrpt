use crate::commands::stacks::models::resource_control::ResourceControl;
use crate::commands::stacks::models::{StackStatus, StackType};
use chrono::serde::ts_seconds::deserialize as from_ts;
use chrono::serde::ts_seconds_option::deserialize as from_ts_option;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct StackList {
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) r#type: StackType,
    pub(crate) status: StackStatus,
    pub(crate) swarm_id: String,
    pub(crate) endpoint_id: u32,
    #[serde(deserialize_with = "from_ts")]
    pub(crate) creation_date: DateTime<Utc>,
    pub(crate) created_by: String,
    #[serde(deserialize_with = "from_ts_option")]
    pub(crate) update_date: Option<DateTime<Utc>>,
    pub(crate) updated_by: Option<String>,
    pub(crate) resource_control: Option<ResourceControl>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stack_list_deserialize() {
        let json = r#"{
            "Id": 1,
            "Name": "my-stack",
            "Type": 2,
            "Status": 1,
            "SwarmId": "",
            "EndpointId": 5,
            "CreationDate": 1700000000,
            "CreatedBy": "admin",
            "UpdateDate": 1700001000,
            "UpdatedBy": "admin",
            "ResourceControl": {
                "Id": 10,
                "ResourceId": "1_my-stack",
                "Type": 6,
                "UserAccesses": [],
                "TeamAccesses": [],
                "Public": true
            }
        }"#;
        let stack: StackList = serde_json::from_str(json).unwrap();
        assert_eq!(stack.id, 1);
        assert_eq!(stack.name, "my-stack");
        assert!(stack.resource_control.is_some());
    }

    #[test]
    fn stack_list_deserialize_null_resource_control() {
        let json = r#"{
            "Id": 2,
            "Name": "no-rc-stack",
            "Type": 1,
            "Status": 2,
            "SwarmId": "",
            "EndpointId": 1,
            "CreationDate": 1700000000,
            "CreatedBy": "admin",
            "UpdateDate": null,
            "UpdatedBy": null,
            "ResourceControl": null
        }"#;
        let stack: StackList = serde_json::from_str(json).unwrap();
        assert_eq!(stack.id, 2);
        assert!(stack.resource_control.is_none());
    }

    #[test]
    fn stack_list_deserialize_null_update_date() {
        let json = r#"{
            "Id": 2,
            "Name": "test",
            "Type": 1,
            "Status": 2,
            "SwarmId": "abc123",
            "EndpointId": 1,
            "CreationDate": 1700000000,
            "CreatedBy": "user",
            "UpdateDate": null,
            "UpdatedBy": null,
            "ResourceControl": {
                "Id": 5,
                "ResourceId": "2_test",
                "Type": 6,
                "UserAccesses": [],
                "TeamAccesses": [],
                "Public": false
            }
        }"#;
        let stack: StackList = serde_json::from_str(json).unwrap();
        assert_eq!(stack.id, 2);
        assert!(stack.resource_control.is_some());
    }
}
