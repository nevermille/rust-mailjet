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

use crate::data::{Contact, ContactsList, MessageData, MessageHistoryData, MessageInformationData};

/// The response base for multiple routes
pub mod generic_response;
/// The response to email sending
mod send_response;
/// A response from Mailjet
pub mod response;

/// The response to message history retrieving
pub type MessageHistoryResponse = generic_response::GenericResponse<MessageHistoryData>;
/// The response to message information retrieving
pub type MessageInformationResponse = generic_response::GenericResponse<MessageInformationData>;
/// The response to message information retrieving
pub type MessageResponse = generic_response::GenericResponse<MessageData>;
/// The response to contact retrieving
pub type ContactResponse = generic_response::GenericResponse<Contact>;
/// The response to contact list retrieving
pub type ContactsListResponse = generic_response::GenericResponse<ContactsList>;

pub use send_response::SendResponse;
pub use response::Response;
