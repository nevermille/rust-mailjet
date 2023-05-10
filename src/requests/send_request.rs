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

use crate::data::Message;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The email sending request
#[doc = include_str!("../../doc/send_basic_email.md")]
#[doc = include_str!("../../doc/send_with_attached_files.md")]
#[doc = include_str!("../../doc/send_in_bulk.md")]
#[doc = include_str!("../../doc/set_global_properties.md")]
#[doc = include_str!("../../doc/use_a_template.md")]
#[doc = include_str!("../../doc/add_email_headers.md")]
#[doc = include_str!("../../doc/tag_email_messages.md")]
#[doc = include_str!("../../doc/group_into_a_campaign.md")]
#[doc = include_str!("../../doc/add_url_tags.md")]
#[derive(Serialize, Deserialize, Default)]
pub struct SendRequest {
    /// The list of all emails to send
    #[serde(rename = "Messages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub messages: Vec<Message>,

    /// When true, enables additional error checks relating to the Send API v3.1 payload
    #[serde(rename = "AdvanceErrorHandling")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advance_error_handling: Option<bool>,

    /// An object containing properties that will be applied to all message objects
    ///
    /// Use this whenever you have property values that are the same across multiple messages in the payload.
    /// For more information and examples <https://dev.mailjet.com/email/guides/send-api-v31/#set-global-payload-properties>
    #[serde(rename = "Globals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub globals: Option<Value>,

    /// Activates the sandbox mode where no email is sent
    #[serde(rename = "SandboxMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox_mode: Option<bool>,
}
