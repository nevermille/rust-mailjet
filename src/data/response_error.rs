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

/// A sending error
///
/// When you get one, then mail has not been sent
#[derive(Serialize, Deserialize, Default)]
pub struct ResponseError {
    /// The error's unique id
    ///
    /// Example of an id: `f987008f-251a-4dff-8ffc-40f1583ad7bc`
    #[serde(rename = "ErrorIdentifier")]
    #[serde(default)]
    pub error_identifier: String,

    /// The error code
    ///
    /// Mailjet uses a list of error codes defined here:
    /// <https://dev.mailjet.com/email/guides/send-api-v31/#sandbox-mode>
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    pub error_code: String,

    /// The status code
    ///
    /// Mailjet uses a list of status codes defined here:
    /// <https://dev.mailjet.com/email/guides/send-api-v31/#sandbox-mode>
    #[serde(rename = "StatusCode")]
    #[serde(default)]
    pub status_code: i64,

    /// A human readable error
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    pub error_message: String,

    /// A list of request fields in error
    #[serde(rename = "ErrorRelatedTo")]
    #[serde(default)]
    pub error_related_to: Vec<String>,
}
