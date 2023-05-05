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

use crate::data::Message;
use serde::{Deserialize, Serialize};

/// The email sending request
#[doc = include_str!("../../doc/send_basic_email.md")]
#[doc = include_str!("../../doc/send_with_attached_files.md")]
#[derive(Serialize, Deserialize, Default)]
pub struct SendRequest {
    /// The list of all emails to send
    #[serde(rename = "Messages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub messages: Vec<Message>,

    /// Activates the sandbox mode where no email is sent
    #[serde(rename = "SandboxMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox_mode: Option<bool>,
}
