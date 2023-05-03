use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{attachment::Attachment, EmailAddress};

#[derive(Serialize, Deserialize, Default)]
pub struct Message {
    #[serde(rename = "From")]
    #[serde(default)]
    pub from: EmailAddress,

    #[serde(rename = "To")]
    #[serde(default)]
    pub to: Vec<EmailAddress>,

    #[serde(rename = "Cc")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cc: Vec<EmailAddress>,

    #[serde(rename = "Bcc")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub bcc: Vec<EmailAddress>,

    #[serde(rename = "Subject")]
    #[serde(default)]
    pub subject: String,

    #[serde(rename = "TextPart")]
    #[serde(default)]
    pub text_part: String,

    #[serde(rename = "HTMLPart")]
    #[serde(default)]
    pub html_part: String,

    #[serde(rename = "TemplateID")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<i64>,

    #[serde(rename = "TemplateLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_language: Option<bool>,

    #[serde(rename = "TemplateErrorReporting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_error_reporting: Option<EmailAddress>,

    #[serde(rename = "TemplateErrorDeliver")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_error_deliver: Option<bool>,

    #[serde(rename = "Attachments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub attachments: Vec<Attachment>,

    #[serde(rename = "InlinedAttachments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub inlined_attachments: Vec<Attachment>,

    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,

    #[serde(rename = "CustomCampaign")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_campain: Option<String>,

    #[serde(rename = "DeduplicateCampaign")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deduplicate_campain: Option<bool>,

    #[serde(rename = "TrackOpens")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_opens: Option<String>,

    #[serde(rename = "TrackClicks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_clicks: Option<String>,

    #[serde(rename = "CustomID")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_id: Option<String>,

    #[serde(rename = "EventPayload")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_payload: Option<String>,

    #[serde(rename = "Headers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub headers: HashMap<String, String>,

    #[serde(rename = "ReplyTo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<EmailAddress>,

    #[serde(rename = "MonitoringCategory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_category: Option<String>,

    #[serde(rename = "Variables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub variables: HashMap<String, String>,
}
