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

use crate::data::{MessageData, MessageHistoryData, MessageInformationData};

/// The response to email sending
mod send_response;
/// The response base for multiple routes
pub mod generic_response;

/// The response to message history retrieving
pub type MessageHistoryResponse = generic_response::GenericResponse<MessageHistoryData>;
/// The response to message information retrieving
pub type MessageInformationResponse = generic_response::GenericResponse<MessageInformationData>;
/// The response to message information retrieving
pub type MessageResponse = generic_response::GenericResponse<MessageData>;

pub use send_response::SendResponse;
