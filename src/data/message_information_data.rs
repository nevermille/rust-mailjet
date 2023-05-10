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
use serde_json::Value;

/// A message infomation
#[derive(Serialize, Deserialize, Default)]
pub struct MessageInformationData {
    /// Unique numeric ID of the campaign this message is part of
    #[serde(rename = "CampaignID")]
    #[serde(default)]
    pub campaign_id: i128,

    /// Number of messages, for which click tracking is enabled
    #[serde(rename = "ClickTrackedCount")]
    #[serde(default)]
    pub click_tracked_count: i64,

    /// Unique numeric ID of the contact, to which the message was sent
    #[serde(rename = "ContactID")]
    #[serde(default)]
    pub contact_id: i128,

    /// Timestamp indicating when the message was created
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    pub created_at: String,

    /// Unique numeric ID of this message
    #[serde(rename = "ID")]
    #[serde(default)]
    pub id: i128,

    /// Size of the message (in bytes)
    #[serde(rename = "MessageSize")]
    #[serde(default)]
    pub message_size: i128,

    /// Number of messages, for which open tracking is enabled
    #[serde(rename = "OpenTrackedCount")]
    #[serde(default)]
    pub open_tracked_count: i64,

    /// Number of messages waiting in the send queue
    #[serde(rename = "QueuedCount")]
    #[serde(default)]
    pub queued_count: i64,

    /// Timestamp indicating when last message was sent for the campaign
    #[serde(rename = "SendEndAt")]
    #[serde(default)]
    pub send_end_at: String,

    /// Number of actual sent attempts
    #[serde(rename = "SentCount")]
    #[serde(default)]
    pub sent_count: i64,

    /// Matched SpamAssassin rules
    #[serde(rename = "SpamAssassinRules")]
    #[serde(default)]
    pub spam_assassin_rules: Value,

    /// SpamAssassin score for this message
    #[serde(rename = "SpamAssassinScore")]
    #[serde(default)]
    pub spam_assassin_score: f64,
}
