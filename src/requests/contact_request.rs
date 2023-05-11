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

/// The contact creation/update request
#[derive(Serialize, Deserialize, Default)]
pub struct ContactRequest {
    /// Indicates whether the contact is added to the exclusion list for campaigns or not
    ///
    /// An excluded contact will not be receiving any marketing emails
    #[serde(rename = "IsExcludedFromCampaigns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_excluded_from_campaigns: Option<bool>,

    /// User-selected name for this contact
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Contact email address
    ///
    /// This field is mandatory for creation, ignored for update
    #[serde(rename = "Email")]
    #[serde(default)]
    pub email: Option<String>,
}
