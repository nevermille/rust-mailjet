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

use super::{response_error::ResponseError, response_success::ResponseSuccess};
use serde::{Deserialize, Serialize};

/// The status of an email sending
///
/// Each message corresponds to one email, no matter how many recipients were present
#[derive(Serialize, Deserialize, Default)]
pub struct ResponseMessage {
    /// The status, can be `success`or `error`
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,

    /// A list of all errors, in case of a `success` status, it should be empty
    #[serde(rename = "Errors")]
    #[serde(default)]
    pub errors: Vec<ResponseError>,

    /// The custom message id you set
    #[serde(rename = "CustomID")]
    #[serde(default)]
    pub custom_id: String,

    /// Each email sent information, one per `To` recipient
    #[serde(rename = "To")]
    #[serde(default)]
    pub to: Vec<ResponseSuccess>,

    /// Each email sent information, one per `Cc` recipient
    #[serde(rename = "Cc")]
    #[serde(default)]
    pub cc: Vec<ResponseSuccess>,

    /// Each email sent information, one per `Bcc` recipient
    #[serde(rename = "Bcc")]
    #[serde(default)]
    pub bcc: Vec<ResponseSuccess>,
}
