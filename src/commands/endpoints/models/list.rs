use serde::{Deserialize, Serialize};
use crate::commands::endpoints::models::{EndpointStatus, EndpointType};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct EndpointList {
    id: u32,
    name: String,
    r#type: EndpointType,
    status: EndpointStatus,
}
