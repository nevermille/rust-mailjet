use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct ResponseError {
    #[serde(rename = "ErrorIdentifier")]
    #[serde(default)]
    pub error_identifier: String,

    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    pub error_code: String,

    #[serde(rename = "StatusCode")]
    #[serde(default)]
    pub status_code: i64,

    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    pub error_message: String,

    #[serde(rename = "ErrorRelatedTo")]
    #[serde(default)]
    pub error_related_to: Vec<String>,
}
