use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct ResponseSuccess {
    #[serde(rename = "Email")]
    #[serde(default)]
    pub email: String,

    #[serde(rename = "MessageUUID")]
    #[serde(default)]
    pub message_uuid: String,

    #[serde(rename = "MessageID")]
    #[serde(default)]
    pub message_id: i128,

    #[serde(rename = "MessageHref")]
    #[serde(default)]
    pub message_href: String,
}
