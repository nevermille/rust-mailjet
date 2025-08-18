use serde::{Deserialize, Serialize};

/// A spam event feedback
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpamFeedback {
    /// Unix timestamp of event
    #[serde(default)]
    pub time: i64,

    /// The unique message ID
    #[serde(default)]
    #[serde(rename = "MessageID")]
    pub message_id: i64,

    /// The unique message GUID
    #[serde(default)]
    #[serde(rename = "Message_GUID")]
    pub message_guid: String,

    /// Email address of recipient triggering the event
    #[serde(default)]
    pub email: String,

    /// Internal Mailjet campaign ID associated to the message
    #[serde(default)]
    pub mj_campaign_id: i64,

    /// Internal Mailjet contact ID
    #[serde(default)]
    pub mj_contact_id: i64,

    /// Value of the X-Mailjet-Campaign header when provided
    #[serde(default)]
    #[serde(rename = "customcampaign")]
    pub custom_campaign: String,

    /// The custom ID, when provided at send time
    #[serde(default)]
    #[serde(rename = "CustomID")]
    pub custom_id: String,

    /// The event payload, when provided at send time
    #[serde(default)]
    #[serde(rename = "Payload")]
    pub payload: String,

    /// Indicates which feedback loop program reported this complaint
    #[serde(default)]
    pub source: String,
}
