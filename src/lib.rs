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

// I know it's annoying, but having undocumented code is out of the question
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

//! Rust wrapper for Mailjet's API
//!
//! Mailjet is a service provider for sending emails and SMS, visit <https://www.mailjet.com/>
//! for more information.
//!
//! **WARNING**: This wrapper is not official, Mailjet won't provide any support for it.
//!
//! # Send a basic email
//!
//! ```
//! use mailjet_api_wrapper::{
//!     data::{EmailAddress, Message},
//!     requests::SendRequest,
//!     Mailjet,
//! };
//!
//! // Create mailjet client
//! let mailjet = Mailjet::from_api_keys("your_key", "your_secret");
//!
//! // Create recipients
//! let to = EmailAddress::from_email("passenger1@mailjet.com");
//! let from = EmailAddress::from_email_and_name("pilot@mailjet.com", "Mailjet Pilot");
//!
//! // Create message
//! let mut message = Message::default();
//! message.to.push(to);
//! message.from = from;
//! message.html_part = "<h3>Dear passenger 1, welcome to <a href=\"https://www.mailjet.com/\">Mailjet</a>!</h3><br />May the delivery force be with you!".to_string();
//! message.text_part = "Dear passenger 1, welcome to Mailjet! May the delivery force be with you!".to_string();
//! message.subject = "Your email flight plan!".to_string();
//!
//! // Create send request
//! let mut send_request = SendRequest::default();
//! send_request.sandbox_mode = Some(true); // You can remove this when sending for real
//! send_request.messages.push(message);
//!
//! // Send emails
//! let response = mailjet.send(&send_request).unwrap();
//! ```
//!
//! # The data structures
//!
//! The request and response structures are the same as mailjet's JSONs with PascalCase field names
//! converted into snake_case format as asked by rust. Everything is serializable/deserializable with
//! serde, so you can easily go back to original JSON formats with serde_json.
//!
//! For more information on JSON structures, go read <https://dev.mailjet.com/email/guides/send-api-v31/>

/// The data types
pub mod data;
/// The mailjet client
mod mailjet;
/// The request structures
pub mod requests;
/// The response structures
pub mod responses;

pub use mailjet::Mailjet;
