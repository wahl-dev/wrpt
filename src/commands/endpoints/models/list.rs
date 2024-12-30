use crate::commands::endpoints::models::{EndpointStatus, EndpointType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct EndpointList {
    id: u32,
    name: String,
    r#type: EndpointType,
    status: EndpointStatus,
}
