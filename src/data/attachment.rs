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

/// An email attachment
#[derive(Serialize, Deserialize, Default)]
pub struct Attachment {
    /// Defines the type of content being sent out using a MIME type
    ///
    /// See <https://www.iana.org/assignments/media-types/media-types.xhtml> for additional information
    #[serde(rename = "ContentType")]
    #[serde(default)]
    pub content_type: String,

    /// The full name of the file (including the file extension)
    #[serde(rename = "Filename")]
    #[serde(default)]
    pub filename: String,

    /// Base64 encoded content of the attached file
    #[serde(rename = "Base64Content")]
    #[serde(default)]
    pub base64_content: String,

    /// Name of the cid to be inserted in the HTML content of the message
    ///
    /// The value must be unique across all inline attachments in the message
    #[serde(rename = "ContentID")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_id: Option<String>,
}
