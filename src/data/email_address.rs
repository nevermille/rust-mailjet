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

/// An email address
#[derive(Serialize, Deserialize, Default)]
pub struct EmailAddress {
    /// The email address
    #[serde(rename = "Email")]
    #[serde(default)]
    pub email: String,

    /// The recipient's name, can be empty
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

impl EmailAddress {
    /// Creates a recipient with only an address
    ///
    /// # Parameters
    ///
    /// * `email`: The recipient's address
    pub fn from_email(email: &str) -> Self {
        Self {
            email: email.to_string(),
            ..Default::default()
        }
    }

    /// Creates a recipient with an address and a name
    ///
    /// # Parameters
    ///
    /// * `email`: The recipient's address
    /// * `name`: The recipient's name
    pub fn from_email_and_name(email: &str, name: &str) -> Self {
        Self {
            email: email.to_string(),
            name: name.to_string(),
        }
    }

    /// Returns `true` if no information were entered, `false` otherwise
    pub fn is_empty(&self) -> bool {
        self.name.is_empty() && self.email.is_empty()
    }
}
