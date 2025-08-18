use serde::{Deserialize, Serialize};

/// A bounce event feedback
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BounceFeedback {
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

    /// `true` if this bounce leads to the recipient being blocked
    #[serde(default)]
    pub blocked: bool,

    /// `true` if error was permanent
    #[serde(default)]
    pub hard_bounce: bool,

    /// See [error table](https://dev.mailjet.com/email/guides/webhooks/#possible-values-for-errors)
    #[serde(default)]
    pub error_related_to: String,

    /// See [error table](https://dev.mailjet.com/email/guides/webhooks/#possible-values-for-errors)
    #[serde(default)]
    pub error: String,

    /// The raw SMTP error code, including descriptions of the reason for the bounce
    #[serde(default)]
    pub comment: String,
}
