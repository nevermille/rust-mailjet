use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Attachment {
    #[serde(rename = "ContentType")]
    #[serde(default)]
    pub content_type: String,

    #[serde(rename = "Filename")]
    #[serde(default)]
    pub filename: String,

    #[serde(rename = "Base64Content")]
    #[serde(default)]
    pub base64_content: String,
}
