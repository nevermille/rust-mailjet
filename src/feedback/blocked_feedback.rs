use serde::{Deserialize, Serialize};

/// A blocked event feedback
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockedFeedback {
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

    /// The custom ID, when provided at send time
    #[serde(default)]
    #[serde(rename = "CustomID")]
    custom_id: String,

    /// The event payload, when provided at send time
    #[serde(default)]
    #[serde(rename = "Payload")]
    payload: String,

    /// See [error table](https://dev.mailjet.com/email/guides/webhooks/#possible-values-for-errors)
    #[serde(default)]
    error_related_to: String,

    /// See [error table](https://dev.mailjet.com/email/guides/webhooks/#possible-values-for-errors)
    #[serde(default)]
    error: String,
}
