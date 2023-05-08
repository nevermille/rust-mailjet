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

use serde::{Deserialize, Serialize};

/// One step of an email's history
#[derive(Serialize, Deserialize, Default)]
pub struct MessageHistoryData {
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
