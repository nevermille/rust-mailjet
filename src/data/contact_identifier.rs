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

/// The contact identifier, can be an id or an email address
pub enum ContactIdentifier {
    /// Unique numeric ID of the contact you want to retrieve
    ContactId(i128),
    /// The email address of the contact your want to retrieve
    ContactEmail(String),
}

impl ToString for ContactIdentifier {
    fn to_string(&self) -> String {
        match self {
            ContactIdentifier::ContactId(v) => v.to_string(),
            ContactIdentifier::ContactEmail(v) => v.to_string(),
        }
    }
}
