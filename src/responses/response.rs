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

/// A response from Mailjet
pub struct Response<T> {
    /// The HTTP Response code
    pub http_code: Option<u32>,

    /// The data as directly returned by Mailjet
    pub raw_data: String,

    /// The parsed data as an object, only if parsing was successful
    pub object: Option<T>
}

impl<T> Response<T> {
    /// Creates a new object
    ///
    /// # Parameters
    ///
    /// * `http_code`: The HTTP response code if supported
    /// * `raw_data` : The response body
    /// * `object` : The parsed object if parsing was successful
    pub fn create_from_data(http_code: Option<u32>, raw_data: String, object: Option<T>) -> Self {
        Self {
            http_code,
            raw_data,
            object
        }
    }
}
