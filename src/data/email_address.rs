use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct EmailAddress {
    #[serde(rename = "Email")]
    #[serde(default)]
    pub email: String,

    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}
