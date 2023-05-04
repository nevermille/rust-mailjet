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

use url_builder::URLBuilder;

/// The message searching request
#[derive(Default)]
pub struct MessageRequest {
    /// Retrieves only messages sent as part of the specified Campaign ID
    pub campaign: Option<i128>,

    /// Retrieves only messages sent to the specified Contact ID
    pub contact: Option<i128>,

    /// Retrieves only messages tagged with the specified `CustomID`
    pub custom_id: Option<String>,

    /// Retrieves only messages sent to the specified Destination ID (ID for a specific domain name)
    pub destination: Option<i128>,

    /// Retrieves only messages sent after the specified timestamp
    ///
    /// Both Unix timestamp (e.g. 1514764800) and RFC3339 (2018-01-01T00:00:00) formats are accepted
    pub from_ts: Option<String>,

    /// Retrieves only messages of the specified type
    ///
    /// See <https://dev.mailjet.com/email/reference/messages/#v3_get_message> for list
    pub from_type: Option<i64>,

    /// Retrieves only non-delivered messages with the respective State ID
    ///
    /// See <https://dev.mailjet.com/email/reference/messages/#v3_get_message> for list
    pub message_state: Option<i64>,

    /// Retrieves only messages with the specified status
    ///
    /// See <https://dev.mailjet.com/email/reference/messages/#v3_get_message> for list
    pub message_status: Option<i64>,

    /// Retrieves only messages with the specified PlanSubscription ID
    pub plan_subscription: Option<i128>,

    /// Retrieves only messages sent using a sender address with the specified ID
    pub sender_id: Option<i128>,

    /// When true, the recipient's email address will be displayed in the message object under `ContactAlt`
    pub show_contact_alt: Option<bool>,

    /// When true, the `CustomID` attached to the message will be displayed in the message object under `CustomID`
    pub show_custom_id: Option<bool>,

    /// When true, the subject of the message will be displayed under `Subject`
    pub show_subject: Option<bool>,

    /// Retrieves only messages sent before the specified timestamp
    ///
    /// Both Unix timestamp (e.g. 1514764800) and RFC3339 (2018-01-01T00:00:00) formats are accepted
    pub to_ts: Option<String>,

    /// Limit the response to a select number of returned objects
    pub limit: Option<i64>,

    /// Retrieve a list of objects starting from a certain offset
    ///
    /// Combine this query parameter with `limit` to retrieve a specific section of the list of objects
    pub offset: Option<i64>,

    /// Set the value of this query parameter to 1 to retrieve the overall number of objects returned
    /// by this request for your API Key
    pub count_only: Option<i8>,

    /// Specify a property name for this query parameter to sort the objects in Data
    pub sort: Option<String>,
}

impl MessageRequest {
    /// Adds parameter to a URL builder
    ///
    /// # Parameters
    ///
    /// * `url_builder`: The URL builder
    pub fn add_parameters_to_url(&self, url_builder: &mut URLBuilder) {
        if let Some(v) = self.campaign {
            url_builder.add_param("Campaign", &v.to_string());
        }

        if let Some(v) = self.contact {
            url_builder.add_param("Contact", &v.to_string());
        }

        if let Some(v) = &self.custom_id {
            url_builder.add_param("CustomID", v);
        }

        if let Some(v) = self.destination {
            url_builder.add_param("Destination", &v.to_string());
        }

        if let Some(v) = &self.from_ts {
            url_builder.add_param("FromTS", v);
        }

        if let Some(v) = self.from_type {
            url_builder.add_param("FromType", &v.to_string());
        }

        if let Some(v) = self.message_state {
            url_builder.add_param("MessageState", &v.to_string());
        }

        if let Some(v) = self.message_status {
            url_builder.add_param("MessageStatus", &v.to_string());
        }

        if let Some(v) = self.plan_subscription {
            url_builder.add_param("PlanSubscription", &v.to_string());
        }

        if let Some(v) = self.sender_id {
            url_builder.add_param("SenderID", &v.to_string());
        }

        if let Some(v) = self.show_contact_alt {
            url_builder.add_param("ShowContactAlt", &v.to_string());
        }

        if let Some(v) = self.show_custom_id {
            url_builder.add_param("ShowCustomID", &v.to_string());
        }

        if let Some(v) = self.show_subject {
            url_builder.add_param("ShowSubject", &v.to_string());
        }

        if let Some(v) = &self.to_ts {
            url_builder.add_param("ToTS", v);
        }

        if let Some(v) = self.limit {
            url_builder.add_param("Limit", &v.to_string());
        }

        if let Some(v) = self.offset {
            url_builder.add_param("Offset", &v.to_string());
        }

        if let Some(v) = self.count_only {
            url_builder.add_param("countOnly", &v.to_string());
        }

        if let Some(v) = &self.sort {
            url_builder.add_param("Sort", v);
        }
    }
}
