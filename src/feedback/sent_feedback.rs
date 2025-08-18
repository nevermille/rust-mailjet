use serde::{Deserialize, Serialize};

/// A sent event feedback
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SentFeedback {
    /// Unix timestamp of event
    #[serde(default)]
    time: i64,

    /// The unique message ID
    #[serde(default)]
    #[serde(rename = "MessageID")]
    message_id: i64,

    /// The unique message GUID
    #[serde(default)]
    #[serde(rename = "Message_GUID")]
    message_guid: String,

    /// Email address of recipient triggering the event
    #[serde(default)]
    email: String,

    /// Internal Mailjet campaign ID associated to the message
    #[serde(default)]
    mj_campaign_id: i64,

    /// Internal Mailjet contact ID
    #[serde(default)]
    mj_contact_id: i64,

    /// Value of the X-Mailjet-Campaign header when provided
    #[serde(default)]
    #[serde(rename = "customcampaign")]
    custom_campaign: String,

    /// The unique message ID as a string
    #[serde(default)]
    #[deprecated(note = "See message_id")]
    mj_message_id: String,

    /// The raw SMTP response message
    #[serde(default)]
    smtp_reply: String,

    /// The custom ID, when provided at send time
    #[serde(default)]
    #[serde(rename = "CustomID")]
    custom_id: String,

    /// The event payload, when provided at send time
    #[serde(default)]
    #[serde(rename = "Payload")]
    payload: String,
}
