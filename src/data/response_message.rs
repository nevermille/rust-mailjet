use serde::{Deserialize, Serialize};

use super::{response_error::ResponseError, response_success::ResponseSuccess};

#[derive(Serialize, Deserialize, Default)]
pub struct ResponseMessage {
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,

    #[serde(rename = "Errors")]
    #[serde(default)]
    pub errors: Vec<ResponseError>,

    #[serde(rename = "CustomID")]
    #[serde(default)]
    pub custom_id: String,

    #[serde(rename = "To")]
    #[serde(default)]
    pub to: Vec<ResponseSuccess>,

    #[serde(rename = "Cc")]
    #[serde(default)]
    pub cc: Vec<ResponseSuccess>,

    #[serde(rename = "Bcc")]
    #[serde(default)]
    pub bcc: Vec<ResponseSuccess>,
}
