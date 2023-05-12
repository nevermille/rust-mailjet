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

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default)]
/// Generic response base appearing in multiple routes
///
/// You shouldn't have to use this structure directly, use type aliases instead
pub struct GenericResponse<T> {
    /// Indicates the number of objects in the `Data` array
    #[serde(rename = "Count")]
    #[serde(default)]
    pub count: i64,

    /// An array containing a list of objects returned by the endpoint
    #[serde(rename = "Data")]
    #[serde(default)]
    pub data: Vec<T>,

    /// Indicates the number of objects in the `Data` array
    #[serde(rename = "Total")]
    #[serde(default)]
    pub total: i64,
}
