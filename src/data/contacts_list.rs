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

/// A contact list information
#[derive(Serialize, Deserialize, Default)]
pub struct ContactsList {
    /// When true, the contact list will be marked as Deleted
    ///
    /// Deleted lists can later be reinstated by updating this value to `false`
    #[serde(rename = "IsDeleted")]
    #[serde(default)]
    pub is_deleted: bool,

    /// User-specified name for this contact list (must be unique)
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,

    /// Unique email address generated by Mailjet, which can be used only via
    /// Mailjet's SMTP server to reach all contacts in the list
    #[serde(rename = "Address")]
    #[serde(default)]
    pub address: String,

    /// Timestamp of when the contact list was created
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    pub created_at: String,

    /// Unique numeric ID assigned to this contact list
    #[serde(rename = "ID")]
    #[serde(default)]
    pub id: i128,

    /// Number of contacts registered in this contact list
    ///
    /// Includes contacts that were unsubscribed from the list, as well as excluded ones
    #[serde(rename = "SubscriberCount")]
    #[serde(default)]
    pub subscriber_count: i64,
}
