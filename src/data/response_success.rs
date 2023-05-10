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

/// A successfully sent email
#[derive(Serialize, Deserialize, Default)]
pub struct ResponseSuccess {
    /// The email address where this email has been sent
    #[serde(rename = "Email")]
    #[serde(default)]
    pub email: String,

    /// The mailjet's email unique id
    ///
    /// Example of an id: `cb927469-36fd-4c02-bce4-0d199929a207`
    #[serde(rename = "MessageUUID")]
    #[serde(default)]
    pub message_uuid: String,

    /// The legacy mailjet's email unique id
    ///
    /// Example of an id: `70650219165027410`
    #[serde(rename = "MessageID")]
    #[serde(default)]
    pub message_id: i128,

    /// The URL to message info
    ///
    /// Example of a URL: `https://api.mailjet.com/v3/REST/message/70650219165027410`
    #[serde(rename = "MessageHref")]
    #[serde(default)]
    pub message_href: String,
}
