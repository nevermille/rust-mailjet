// This file is part of rust-mailjet <https://github.com/nevermille/rust-mailjet>
// Copyright (C) 2023 Camille Nevermind

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use super::{attachment::Attachment, EmailAddress};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// An email
#[derive(Serialize, Deserialize, Default)]
pub struct Message {
    /// The email's sender
    ///
    /// Must be a valid, activated and registered sender for this account
    #[serde(rename = "From")]
    #[serde(default)]
    pub from: EmailAddress,

    /// The main recipients
    ///
    /// If a recipient is specified twice, it is counted only once
    #[serde(rename = "To")]
    #[serde(default)]
    pub to: Vec<EmailAddress>,

    /// The copy recipients
    ///
    /// If a recipient is specified twice, it is counted only once
    #[serde(rename = "Cc")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cc: Vec<EmailAddress>,

    /// The hidden copy recipients
    ///
    /// If a recipient is specified twice, it is counted only once
    #[serde(rename = "Bcc")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub bcc: Vec<EmailAddress>,

    /// The email's subject
    #[serde(rename = "Subject")]
    #[serde(default)]
    pub subject: String,

    /// The email's body in plain text form
    ///
    /// Developper's note: please fill this field for visualy impaired people
    #[serde(rename = "TextPart")]
    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub text_part: String,

    /// The email's body in HTML format
    #[serde(rename = "HTMLPart")]
    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub html_part: String,

    /// If you're not providing the body, you can use a template here
    #[serde(rename = "TemplateID")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<i64>,

    /// The Template ID or Name to use as this email content
    ///
    /// Overrides the HTML/Text parts if any.
    #[serde(rename = "TemplateLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_language: Option<bool>,

    /// Email Address where a carbon copy with error message is sent to
    #[serde(rename = "TemplateErrorReporting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_error_reporting: Option<EmailAddress>,

    /// Define if the message is delivered if an error is discovered in the templating language
    ///
    /// By default the delivery is deactivated.
    #[serde(rename = "TemplateErrorDeliver")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_error_deliver: Option<bool>,

    /// Standard file attachments
    #[serde(rename = "Attachments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub attachments: Vec<Attachment>,

    /// File attachments for inline use via `cid:FILENAME.EXT`
    #[serde(rename = "InlinedAttachments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub inlined_attachments: Vec<Attachment>,

    /// Manage message processing priority inside your account (API key) scheduling queue
    ///
    /// Default is `2`
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,

    /// Groups multiple messages in one campaign
    #[serde(rename = "CustomCampaign")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_campain: Option<String>,

    /// Block/unblock messages to be sent multiple times inside one campaign to the same contact
    #[serde(rename = "DeduplicateCampaign")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deduplicate_campain: Option<bool>,

    /// Force or disable open tracking on this message
    ///
    /// Can override account preferences. Can only be used with a HTML part.
    ///
    /// * `0`: take the values defined on the account
    /// * `1`: disable the tracking
    /// * `2`: enable the tracking
    #[serde(rename = "TrackOpens")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_opens: Option<String>,

    /// Force or disable click tracking on this message
    ///
    /// Can override account preferences. Can only be used with a HTML part.
    ///
    /// * `0`: take the values defined on the account
    /// * `1`: disable the tracking
    /// * `2`: enable the tracking
    #[serde(rename = "TrackClicks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_clicks: Option<String>,

    /// Attach a custom ID to the message
    #[serde(rename = "CustomID")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_id: Option<String>,

    /// Attach a payload to the message
    #[serde(rename = "EventPayload")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_payload: Option<String>,

    /// Add lines to the email's headers
    #[serde(rename = "Headers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub headers: HashMap<String, String>,

    /// The email address for replying
    #[serde(rename = "ReplyTo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<EmailAddress>,

    /// Category for Real-time Monitoring to which the message will be attached
    #[serde(rename = "MonitoringCategory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_category: Option<String>,

    /// Global variables used for personalization
    #[serde(rename = "Variables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub variables: HashMap<String, String>,
}
