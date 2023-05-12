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

/// The contact list retrieving request
#[derive(Default)]
pub struct ContactsListSearchRequest {
    /// Retrieves only the contact list with the specified unique address created by Mailjet
    pub address: Option<String>,

    /// Excludes the contact list with specified ID from the response
    pub exclude_id: Option<i128>,

    /// When true, will retrieve only deleted lists
    pub is_deleted: Option<bool>,

    /// Retrieves only the list with the specified user-selected Name
    pub name: Option<String>,

    /// Limit the response to a select number of returned objects
    pub limit: Option<i64>,

    /// Retrieve a list of objects starting from a certain offset
    pub offset: Option<i64>,

    /// Set the value of this query parameter to 1 to retrieve the overall
    /// number of objects returned by this request
    pub count_only: Option<i8>,

    /// Specify a property name for this query parameter to sort the objects in `data`
    pub sort: Option<String>,
}

impl UrlEncodedRequest for ContactsListSearchRequest {
    fn add_parameters_to_url(&self, url_builder: &mut URLBuilder) {
        if let Some(v) = &self.address {
            url_builder.add_param("Address", v.as_str());
        }

        if let Some(v) = &self.exclude_id {
            url_builder.add_param("Address", v.to_string().as_str());
        }

        if let Some(v) = &self.is_deleted {
            url_builder.add_param("ExcludeID", v.to_string().as_str());
        }

        if let Some(v) = &self.name {
            url_builder.add_param("IsDeleted", v.as_str());
        }

        if let Some(v) = &self.limit {
            url_builder.add_param("Limit", v.to_string().as_str());
        }

        if let Some(v) = &self.offset {
            url_builder.add_param("Offset", v.to_string().as_str());
        }

        if let Some(v) = &self.count_only {
            url_builder.add_param("countOnly", v.to_string().as_str());
        }

        if let Some(v) = &self.sort {
            url_builder.add_param("Sort", v.as_str());
        }
    }
}
