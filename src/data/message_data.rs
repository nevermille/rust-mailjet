// This file is part of rust-mailjet <https://github.com/nevermille/rust-mailjet>
// Copyright (C) 2023 Camille Nevermind
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; either
// version 3 of the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with this program; if not, write to the Free Software Foundation,
// Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.

use serde::{Deserialize, Serialize};

/// An email information
#[derive(Serialize, Deserialize, Default)]
pub struct MessageData {
    /// Timestamp indicating when the message arrived in the recipient's mailbox
    #[serde(rename = "ArrivedAt")]
    #[serde(default)]
    pub arrived_at: String,

    /// Number of attachments detected for this message
    #[serde(rename = "AttachmentCount")]
    #[serde(default)]
    pub attachment_count: i64,

    /// Number of attempts made to deliver this message
    #[serde(rename = "AttemptCount")]
    #[serde(default)]
    pub attempt_count: i64,

    /// Unique numeric ID for the campaign this message is part of
    #[serde(rename = "CampaignID")]
    #[serde(default)]
    pub campaign_id: i128,

    /// The email address of the contact, to which the message was sent
    #[serde(rename = "ContactAlt")]
    #[serde(default)]
    pub contact_alt: String,

    /// Unique numeric ID for the contact, to which the message was sent
    #[serde(rename = "ContactID")]
    #[serde(default)]
    pub contact_id: i128,

    /// Delay between the message being processed and it being delivered (in milliseconds)
    #[serde(rename = "Delay")]
    #[serde(default)]
    pub delay: f64,

    /// Unique numeric ID of the recipient email's domain
    #[serde(rename = "DestinationID")]
    #[serde(default)]
    pub destination_id: i128,

    /// Time spent processing the text of the message (in milliseconds)
    #[serde(rename = "FilterTime")]
    #[serde(default)]
    pub filter_time: i64,

    /// Unique numeric ID of this message
    #[serde(rename = "ID")]
    #[serde(default)]
    pub id: i128,

    /// Indicates whether clicks are tracked for this message or not
    #[serde(rename = "IsClickTracked")]
    #[serde(default)]
    pub is_click_tracked: bool,

    /// Indicates whether the message includes any HTML content or not
    #[serde(rename = "IsHTMLPartIncluded")]
    #[serde(default)]
    pub is_html_part_included: bool,

    /// Indicates whether opens are tracked for this message or not
    #[serde(rename = "IsOpenTracked")]
    #[serde(default)]
    pub is_open_tracked: bool,

    /// Indicates whether the message includes a plain text part or not
    #[serde(rename = "IsTextPartIncluded")]
    #[serde(default)]
    pub is_text_part_included: bool,

    /// Indicates whether unsubscriptions are tracked for this message or not
    #[serde(rename = "IsUnsubTracked")]
    #[serde(default)]
    pub is_unsub_tracked: bool,

    /// Indicates the message size (in bytes)
    #[serde(rename = "MessageSize")]
    #[serde(default)]
    pub message_size: i64,

    /// Unique numeric ID of the sender email address
    #[serde(rename = "SenderID")]
    #[serde(default)]
    pub sender_id: i128,

    /// SpamAssassin score for this message
    #[serde(rename = "SpamassassinScore")]
    #[serde(default)]
    pub spamassassin_score: f64,

    /// Matched SpamAssassin rules
    #[serde(rename = "SpamassRules")]
    #[serde(default)]
    pub spamass_rules: String,

    /// Unique numeric ID explaining why the message was not delivered successfully to the recipient
    ///
    /// See <https://dev.mailjet.com/email/reference/messages/#v3_get_message>
    #[serde(rename = "StateID")]
    #[serde(default)]
    pub state_id: i64,

    /// Indicates whether the current state of the message is permanent or not
    #[serde(rename = "StatePermanent")]
    #[serde(default)]
    pub state_permanent: bool,

    /// Current message status
    ///
    /// See <https://dev.mailjet.com/email/reference/messages/#v3_get_message>
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,

    /// The subject line for this message
    #[serde(rename = "Subject")]
    #[serde(default)]
    pub subject: String,

    /// Unique 128-bit ID for this message
    #[serde(rename = "UUID")]
    #[serde(default)]
    pub uuid: String,
}
