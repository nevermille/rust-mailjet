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

use crate::{requests::SendRequest, responses::SendResponse};
use curl::{
    easy::{Easy, List},
    Error,
};
use std::{error::Error as StdError, io::Read};

/// The mailjet client
pub struct Mailjet {
    /// The API key
    pub api_key: String,

    /// The secret key
    pub api_secret: String,
}

impl Mailjet {
    /// Creates a new instance with API keys
    /// You can get yours at <https://app.mailjet.com/account/apikeys>
    ///
    /// # Parameters
    ///
    /// * `key`: The API key
    /// * `secret`: The secret key
    pub fn from_api_keys(key: &str, secret: &str) -> Self {
        Self {
            api_key: key.to_string(),
            api_secret: secret.to_string(),
        }
    }

    /// Executes an API call to a URL
    ///
    /// # Parameters
    ///
    /// * `url`: The URL where to request
    /// * `data`: The data to write in the request's body
    /// * `post`: If the request needs to be done with POST method
    fn exec(&self, url: &str, data: &str, post: bool) -> Result<String, Error> {
        let mut curl = Easy::new();
        let mut response: Vec<u8> = Vec::new(); // That's where the response will be written on

        // Convert the data in a byte array
        let data_json_string = data.to_string();
        let mut raw_data = data_json_string.as_str().as_bytes();

        // Create the HTTP request
        curl.url(url)?;
        curl.username(self.api_key.as_str())?;
        curl.password(self.api_secret.as_str())?;
        curl.post(post)?;

        let mut header_list = List::new();
        header_list.append("Content-Type: application/json")?;
        curl.http_headers(header_list)?;

        {
            // We need this for lifetime reasons
            let mut transfer = curl.transfer();

            // How we pass data to mailjet
            transfer.read_function(|buffer| Ok(raw_data.read(buffer).unwrap_or_default()))?;

            // How we read mailjet's response
            transfer.write_function(|buffer| {
                let _ = &response.extend_from_slice(buffer);
                Ok(buffer.len())
            })?;

            // Request execution
            transfer.perform()?;
        }

        Ok(String::from_utf8_lossy(&response).to_string())
    }

    /// Sends emails via Send API v3.1
    ///
    /// # Parameters
    ///
    /// * `request`: The request containing all emails
    pub fn send(&self, request: &SendRequest) -> Result<SendResponse, Box<dyn StdError>> {
        let j = serde_json::to_string(request)?;
        let response = self.exec("https://api.mailjet.com/v3.1/send", &j, true)?;
        Ok(serde_json::from_str(&response)?)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        data::{EmailAddress, Message},
        requests::SendRequest,
        Mailjet,
    };

    #[test]
    fn send_message() {
        let mut send_request = SendRequest::default();
        let to = EmailAddress::from_email("john.doe@example.com");
        let from = EmailAddress::from_email_and_name("jane.doe@example.com", "Jane Doe");
        let mut message = Message::default();
        let mailjet = Mailjet::from_api_keys(
            &std::env::var("MJ_KEY").unwrap(),
            &std::env::var("MJ_SECRET").unwrap(),
        );

        message.to.push(to);
        message.from = from;
        message.html_part = "<p>Hello world!</p>".to_string();
        message.text_part = "Hello World!".to_string();
        message.subject = "Test email".to_string();

        send_request.sandbox_mode = Some(true);
        send_request.messages.push(message);

        let response = mailjet.send(&send_request).unwrap();

        assert_eq!(response.messages.len(), 1);
        assert_eq!(response.messages.get(0).unwrap().status, "success");
        assert_eq!(response.messages.get(0).unwrap().to.len(), 1);
        assert_eq!(
            response.messages.get(0).unwrap().to.get(0).unwrap().email,
            "john.doe@example.com"
        );
    }
}
