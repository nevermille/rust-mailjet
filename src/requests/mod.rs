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

/// The contact creation/update request
mod contact_request;
/// The contact retrieving request
mod contact_search_request;
/// The contact list creation/update request
mod contacts_list_request;
/// The contact list retrieving request
mod contacts_list_search_request;
/// The message information searching request
mod message_information_request;
/// The message searching request
mod message_request;
/// The email sending request
mod send_request;

pub use contact_request::ContactRequest;
pub use contact_search_request::ContactSearchRequest;
pub use contacts_list_request::ContactsListRequest;
pub use contacts_list_search_request::ContactsListSearchRequest;
pub use message_information_request::MessageInformationRequest;
pub use message_request::MessageRequest;
pub use send_request::SendRequest;
