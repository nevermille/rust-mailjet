use crate::data::Message;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct SendRequest {
    #[serde(rename = "Messages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub messages: Vec<Message>,

    #[serde(rename = "SandboxMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox_mode: Option<bool>,
}
