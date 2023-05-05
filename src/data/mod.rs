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

/// An email attachment
mod attachment;
/// An email address
mod email_address;
/// An email
mod message;
/// An email information
mod message_data;
/// One step of an email's history
mod message_history_data;
/// A message infomation
mod message_information_data;
/// A sending error
mod response_error;
/// The status of an email sending
mod response_message;
/// A successfully sent email
mod response_success;

pub use attachment::Attachment;
pub use email_address::EmailAddress;
pub use message::Message;
pub use message_data::MessageData;
pub use message_history_data::MessageHistoryData;
pub use message_information_data::MessageInformationData;
pub use response_error::ResponseError;
pub use response_message::ResponseMessage;
pub use response_success::ResponseSuccess;
