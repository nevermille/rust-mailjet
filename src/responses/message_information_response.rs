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

use crate::data::MessageInformationData;
use serde::{Deserialize, Serialize};

/// The response to message information retrieving
#[derive(Serialize, Deserialize, Default)]
pub struct MessageInformationResponse {
    /// Indicates the number of objects in the `Data` array
    #[serde(rename = "Count")]
    #[serde(default)]
    pub count: i64,

    /// An array containing a list of objects returned by the endpoint
    #[serde(rename = "Data")]
    #[serde(default)]
    pub data: Vec<MessageInformationData>,

    /// Indicates the number of objects in the `Data` array
    #[serde(rename = "Total")]
    #[serde(default)]
    pub total: i64,
}
