use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
/// One step of an email's history
pub struct MessageHistoryInformation {
    /// A comment providing additional details in case of issues with the message delivery
    #[serde(rename = "Comment")]
    #[serde(default)]
    pub comment: String,

    /// Unix timestamp indicating when the event was registered
    #[serde(rename = "EventAt")]
    #[serde(default)]
    pub event_at: i64,

    /// The type of this event
    ///
    /// See <https://dev.mailjet.com/email/reference/messages/#v3_get_messagehistory_message_ID>
    #[serde(rename = "EventType")]
    #[serde(default)]
    pub event_type: String,

    /// When the message is not successfully delivered, will display the reason for non-delivery
    #[serde(rename = "State")]
    #[serde(default)]
    pub state: String,

    /// User agent (browser) used to trigger the event (when applicable)
    #[serde(rename = "Useragent")]
    #[serde(default)]
    pub useragent: String,

    /// Unique numeric ID of the user agent (browser) used to trigger this event
    #[serde(rename = "UseragentID")]
    #[serde(default)]
    pub useragent_id: i128,
}
