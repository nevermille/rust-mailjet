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

/// A contact information
#[derive(Serialize, Deserialize, Default)]
pub struct Contact {
    /// Indicates whether the contact is added to the exclusion list for campaigns or not
    ///
    /// An excluded contact will not be receiving any marketing emails
    #[serde(rename = "IsExcludedFromCampaigns")]
    #[serde(default)]
    pub is_excluded_from_campaigns: bool,

    /// User-selected name for this contact
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,

    /// Indicates when the contact was added to the global contact list
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    pub created_at: String,

    /// Number of messages delivered to this contact
    #[serde(rename = "DeliveredCount")]
    #[serde(default)]
    pub delivered_count: i64,

    /// Contact email address
    #[serde(rename = "Email")]
    #[serde(default)]
    pub email: String,

    /// Timestamp of the last time the exclusion status of this contact has changed
    #[serde(rename = "ExclusionFromCampaignsUpdatedAt")]
    #[serde(default)]
    pub exclusion_from_campaigns_updated_at: String,

    /// Unique numeric ID of this contact
    #[serde(rename = "ID")]
    #[serde(default)]
    pub id: i128,

    /// Indicates whether the contact's subscription to a contact list is pending or not
    #[serde(rename = "IsOptInPending")]
    #[serde(default)]
    pub is_opt_in_pending: bool,

    /// Indicates whether any spam complaints have been received for this contact or not
    #[serde(rename = "IsSpamComplaining")]
    #[serde(default)]
    pub is_spam_complaining: bool,

    /// Timestamp of last registered activity for this contact - receiving an email, open,
    /// click, unsubscribe etc
    #[serde(rename = "LastActivityAt")]
    #[serde(default)]
    pub last_activity_at: String,

    /// Timestamp of the last time this contact's name or exclusion status was changed
    #[serde(rename = "LastUpdateAt")]
    #[serde(default)]
    pub last_update_at: String,
}
