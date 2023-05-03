use crate::data::ResponseMessage;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct SendResponse {
    #[serde(rename = "Messages")]
    #[serde(default)]
    pub messages: Vec<ResponseMessage>,
}
