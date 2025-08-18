use serde::{Deserialize, Serialize};

/// An open event feedback
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenFeedback {
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

    /// IP address (can be IPv4 or IPv6) that triggered the event
    #[serde(default)]
    ip: String,

    /// Country code of IP address (see [list](http://www.maxmind.com/app/iso3166))
    #[serde(default)]
    geo: String,

    /// User-Agent
    #[serde(default)]
    agent: String,
}
