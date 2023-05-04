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

/// An email attachment
#[derive(Serialize, Deserialize, Default)]
pub struct Attachment {
    /// The MIME type
    #[serde(rename = "ContentType")]
    #[serde(default)]
    pub content_type: String,

    /// The filename
    #[serde(rename = "Filename")]
    #[serde(default)]
    pub filename: String,

    /// The file's contents encoded in Base64 format
    #[serde(rename = "Base64Content")]
    #[serde(default)]
    pub base64_content: String,
}
