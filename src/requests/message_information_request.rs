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

/// The message information searching request
#[derive(Default)]
pub struct MessageInformationRequest {
    /// Retrieves information only about messages that are part of this campaign
    pub campaign_id: Option<i128>,

    /// Retrieves information only about messages that were sent to this contact list
    pub contacts_list: Option<i128>,

    /// Retrieves information only about messages that are part of a campaign with the specified Custom ID
    pub custom_campaign: Option<String>,

    /// Retrieves information only about messages sent from the specified sender email address
    pub from: Option<String>,

    /// Retrieves information only about messages sent from the specified sender domain
    pub from_domain: Option<String>,

    /// Retrieves information only about messages sent from the specified sender ID
    pub from_id: Option<i128>,

    /// Retrieves information only about messages sent after the specified timestamp
    ///
    /// Both Unix timestamp (e.g. 1514764800) and RFC3339 (2018-01-01T00:00:00) formats are accepted
    pub from_ts: Option<String>,

    /// Retrieves information only for a specific type of messages
    ///
    /// See <https://dev.mailjet.com/email/reference/messages/#v3_get_messageinformation>
    pub from_type: Option<i64>,

    /// When true, will retrieve information only for messages that are part of deleted campaigns.
    /// When false, messages from deleted campaigns will be excluded from the response.
    pub is_deleted: Option<bool>,

    /// When true, will retrieve information only for campaigns created with the Legacy template builder.
    /// When false, campaigns created with the Legacy builder will be excluded from the response.
    pub is_newsletter_tool: Option<bool>,

    /// When true, will retrieve information only for messages that are part of starred campaigns.
    /// When false, messages from starred campaigns will be excluded from the response.
    pub is_starred: Option<bool>,

    /// Retrieves information only for messages with the specified status
    ///
    /// See <https://dev.mailjet.com/email/reference/messages/#v3_get_messageinformation>
    pub message_status: Option<i64>,

    /// Retrieves information only for messages sent between the start of the selected period
    /// and the current timestamp
    pub period: Option<String>,

    /// Retrieves information only for messages sent before the specified timestamp
    ///
    /// Both Unix timestamp (e.g. 1514764800) and RFC3339 (2018-01-01T00:00:00) formats are accepted
    pub to_ts: Option<String>,

    /// Limit the response to a select number of returned objects
    pub limit: Option<i64>,

    /// Retrieve a list of objects starting from a certain offset
    pub offset: Option<i64>,

    /// Set the value of this query parameter to 1 to retrieve the overall number of objects
    /// returned by this request for your API Key
    pub count_only: Option<i64>,

    /// Specify a property name for this query parameter to sort the objects in `Data`
    pub sort: Option<String>,
}

impl MessageInformationRequest {
    /// Adds parameter to a URL builder
    ///
    /// # Parameters
    ///
    /// * `url_builder`: The URL builder
    pub fn add_parameters_to_url(&self, url_builder: &mut URLBuilder) {
        if let Some(v) = self.campaign_id {
            url_builder.add_param("CampaignID", &v.to_string());
        }

        if let Some(v) = self.contacts_list {
            url_builder.add_param("ContactsList", &v.to_string());
        }

        if let Some(v) = &self.custom_campaign {
            url_builder.add_param("CustomCampaign", v);
        }

        if let Some(v) = &self.from {
            url_builder.add_param("From", v);
        }

        if let Some(v) = &self.from_domain {
            url_builder.add_param("FromDomain", v);
        }

        if let Some(v) = self.from_id {
            url_builder.add_param("FromID", &v.to_string());
        }

        if let Some(v) = &self.from_ts {
            url_builder.add_param("FromTS", v);
        }

        if let Some(v) = self.from_type {
            url_builder.add_param("FromType", &v.to_string());
        }

        if let Some(v) = self.is_deleted {
            url_builder.add_param("IsDeleted", &v.to_string());
        }

        if let Some(v) = self.is_newsletter_tool {
            url_builder.add_param("IsNewsletterTool", &v.to_string());
        }

        if let Some(v) = self.is_starred {
            url_builder.add_param("IsStarred", &v.to_string());
        }

        if let Some(v) = self.message_status {
            url_builder.add_param("MessageStatus", &v.to_string());
        }

        if let Some(v) = &self.period {
            url_builder.add_param("Period", v);
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
