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

use crate::traits::UrlEncodedRequest;
use url_builder::URLBuilder;

/// The contact retrieving request
#[doc = include_str!("../../doc/retrieve_a_contact.md")]
#[derive(Default)]
pub struct ContactSearchRequest {
    /// Retrieves only contacts targeted by this Campaign ID
    pub campaign: Option<i128>,

    /// Retrieves only contacts that are part of this Contact List ID
    pub contacts_list: Option<i128>,

    /// When true, will retrieve only contacts that have been added to the
    /// exclusion list for marketing emails. When false, those contacts will
    /// be excluded from the response
    pub is_excluded_from_campaigns: Option<bool>,

    /// Limit the response to a select number of returned objects
    pub limit: Option<i64>,

    /// Retrieve a list of objects starting from a certain offset
    pub offset: Option<i64>,

    /// Set the value of this query parameter to 1 to retrieve the overall number of objects returned by this request
    pub count_only: Option<i64>,

    /// Specify a property name for this query parameter to sort the objects in `data`
    pub sort: Option<String>,
}

impl UrlEncodedRequest for ContactSearchRequest {
    fn add_parameters_to_url(&self, url_builder: &mut URLBuilder) {
        if let Some(v) = self.campaign {
            url_builder.add_param("Campaign", &v.to_string());
        }

        if let Some(v) = self.contacts_list {
            url_builder.add_param("ContactsList", &v.to_string());
        }

        if let Some(v) = self.is_excluded_from_campaigns {
            url_builder.add_param("IsExcludedFromCampaigns", &v.to_string());
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
