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

use crate::data::{ContactIdentifier, ContactsListIdentifier};
use crate::traits::UrlEncodedRequest;
use crate::{requests::*, responses::*};
use curl::{
    easy::{Easy, List},
    Error,
};
use std::{error::Error as StdError, io::Read};
use url_builder::URLBuilder;
use crate::macros::log::info;

/// The mailjet client
pub struct Mailjet {
    /// The API key
    pub api_key: String,

    /// The secret key
    pub api_secret: String,
}

/// HTTP Methods
pub enum RequestType {
    /// HTTP GET
    Get,

    /// HTTP POST
    Post,

    /// HTTP PUT
    Put,

    /// HTTP DELETE
    Delete,
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

    /// Executes a request to mailjet
    ///
    /// # Parameters
    ///
    /// * `url`: The URL where to request
    /// * `data`: The data to send if any
    /// * `request_type`: The HTTP request type
    fn exec(
        &self,
        url: &str,
        data: Option<String>,
        request_type: RequestType,
    ) -> Result<(String, Result<u32, Error>), Error> {
        let mut curl = Easy::new();
        let mut response: Vec<u8> = Vec::new(); // That's where the response will be written on

        // Extract the data if any for lifetime reasons
        let data_string = data.unwrap_or_default();

        // Convert the data in a byte array
        let mut raw_data = match data_string.is_empty() {
            false => data_string.as_bytes(),
            true => &[],
        };

        // Create the HTTP request
        curl.url(url)?;
        curl.username(self.api_key.as_str())?;
        curl.password(self.api_secret.as_str())?;

        // Change HTTP request
        match request_type {
            RequestType::Post => curl.post(true)?,
            RequestType::Put => curl.put(true)?,
            RequestType::Delete => curl.custom_request("DELETE")?,
            _ => (),
        }

        // Change body type
        if matches!(request_type, RequestType::Post) || matches!(request_type, RequestType::Put) {
            let mut header_list = List::new();
            header_list.append("Content-Type: application/json")?;
            curl.http_headers(header_list)?;
        }

        info!("Sending a request to Mailjet at {}", url);

        {
            // We need this for lifetime reasons
            let mut transfer = curl.transfer();

            // How we pass data to mailjet
            if !raw_data.is_empty() {
                transfer.read_function(|buffer| Ok(raw_data.read(buffer).unwrap_or_default()))?
            };

            // How we read mailjet's response
            transfer.write_function(|buffer| {
                let _ = &response.extend_from_slice(buffer);
                Ok(buffer.len())
            })?;

            // Request execution
            transfer.perform()?;
        }

        Ok((
            String::from_utf8_lossy(&response).to_string(),
            curl.response_code(),
        ))
    }

    /// Executes an API POST call to a URL
    ///
    /// # Parameters
    ///
    /// * `url`: The URL where to request
    fn get(&self, url: &str) -> Result<(String, Result<u32, Error>), Error> {
        self.exec(url, None, RequestType::Get)
    }

    /// Executes an API DELETE call to a URL
    ///
    /// # Parameters
    ///
    /// * `url`: The URL where to request
    fn delete(&self, url: &str) -> Result<(String, Result<u32, Error>), Error> {
        self.exec(url, None, RequestType::Delete)
    }

    /// Executes an API POST call to a URL
    ///
    /// # Parameters
    ///
    /// * `url`: The URL where to request
    /// * `data`: The data to write in the request's body
    fn post(&self, url: &str, data: &str) -> Result<(String, Result<u32, Error>), Error> {
        self.exec(url, Some(data.to_string()), RequestType::Post)
    }

    /// Executes an API PUT call to a URL
    ///
    /// # Parameters
    ///
    /// * `url`: The URL where to request
    /// * `data`: The data to write in the request's body
    fn put(&self, url: &str, data: &str) -> Result<(String, Result<u32, Error>), Error> {
        self.exec(url, Some(data.to_string()), RequestType::Put)
    }

    /// Sends emails via Send API v3.1
    ///
    /// # Parameters
    ///
    /// * `request`: The request containing all emails
    pub fn send(&self, request: &SendRequest) -> Result<Response<SendResponse>, Box<dyn StdError>> {
        let j = serde_json::to_string(request)?;
        let (response, code) = self.post("https://api.mailjet.com/v3.1/send", &j)?;
        let object = serde_json::from_str(&response).ok();
        Ok(Response::create_from_data(code.ok(),response, object))
    }

    /// Gets a list of messages with specific information on the type of content, tracking, sending and delivery
    ///
    /// # Parameters
    ///
    /// * `search`: The search arguments
    pub fn message(&self, search: &MessageRequest) -> Result<Response<MessageResponse>, Box<dyn StdError>> {
        // Create url
        let mut ub = URLBuilder::new();

        ub.set_protocol("https")
            .set_host("api.mailjet.com")
            .add_route("v3")
            .add_route("REST")
            .add_route("message");

        search.add_parameters_to_url(&mut ub);

        // Execute request
        let (response, code) = self.get(&ub.build())?;

        let object = serde_json::from_str(&response).ok();
        Ok(Response::create_from_data(code.ok(),response, object))
    }

    /// Retrieves specific information on the type of content, tracking, sending and
    /// delivery for a specific processed message
    ///
    /// # Parameters
    ///
    /// * `message_id`: The message id
    pub fn message_from_id(&self, message_id: i128) -> Result<Response<MessageResponse>, Box<dyn StdError>> {
        // Create url
        let mut ub = URLBuilder::new();

        ub.set_protocol("https")
            .set_host("api.mailjet.com")
            .add_route("v3")
            .add_route("REST")
            .add_route("message")
            .add_route(&message_id.to_string());

        // Execute request
        let (response, code) = self.get(&ub.build())?;

        let object = serde_json::from_str(&response).ok();
        Ok(Response::create_from_data(code.ok(),response, object))
    }

    /// Retrieve the event history (sending, open, click etc.) for a specific message
    ///
    /// # Parameters
    ///
    /// * `message_id`: The message id
    pub fn message_history_from_id(
        &self,
        message_id: i128,
    ) -> Result<Response<MessageHistoryResponse>, Box<dyn StdError>> {
        // Create url
        let mut ub = URLBuilder::new();

        ub.set_protocol("https")
            .set_host("api.mailjet.com")
            .add_route("v3")
            .add_route("REST")
            .add_route("messagehistory")
            .add_route(&message_id.to_string());

        // Execute request
        let (response, code) = self.get(&ub.build())?;

        let object = serde_json::from_str(&response).ok();
        Ok(Response::create_from_data(code.ok(),response, object))
    }

    /// Retrieve sending / size / spam information about all messages
    ///
    /// # Parameters
    ///
    /// * `search`: The search arguments
    pub fn message_information(
        &self,
        search: &MessageInformationRequest,
    ) -> Result<Response<MessageInformationResponse>, Box<dyn StdError>> {
        // Create url
        let mut ub = URLBuilder::new();

        ub.set_protocol("https")
            .set_host("api.mailjet.com")
            .add_route("v3")
            .add_route("REST")
            .add_route("messageinformation");

        search.add_parameters_to_url(&mut ub);

        // Execute request
        let (response, code) = self.get(&ub.build())?;

        let object = serde_json::from_str(&response).ok();
        Ok(Response::create_from_data(code.ok(),response, object))
    }

    /// Retrieve sending / size / spam information about a specific message ID
    ///
    /// # Parameters
    ///
    /// * `message_id`: The message id
    pub fn message_information_from_id(
        &self,
        message_id: i128,
    ) -> Result<Response<MessageInformationResponse>, Box<dyn StdError>> {
        // Create url
        let mut ub = URLBuilder::new();

        ub.set_protocol("https")
            .set_host("api.mailjet.com")
            .add_route("v3")
            .add_route("REST")
            .add_route("messageinformation")
            .add_route(&message_id.to_string());

        // Execute request
        let (response, code) = self.get(&ub.build())?;

        let object = serde_json::from_str(&response).ok();
        Ok(Response::create_from_data(code.ok(),response, object))
    }

    /// Add a new unique contact to your global contact list and select its exclusion status
    ///
    /// # Parameters
    ///
    /// * `request`: The request containing contact data
    pub fn contact_create(
        &self,
        request: &ContactRequest,
    ) -> Result<Response<
    ContactResponse>, Box<dyn StdError>> {
        let j = serde_json::to_string(request)?;
        let (response, code) = self.post("https://api.mailjet.com/v3/REST/contact", &j)?;
        let object = serde_json::from_str(&response).ok();
        Ok(Response::create_from_data(code.ok(),response, object))
    }

    /// Retrieve a list of all contacts.
    ///
    /// Includes information about contact status and creation / activity timestamps
    ///
    /// # Parameters
    ///
    /// * `search`: The search arguments
    pub fn contact_search(
        &self,
        search: &ContactSearchRequest,
    ) -> Result<Response<ContactResponse>, Box<dyn StdError>> {
        let mut ub = URLBuilder::new();

        ub.set_protocol("https")
            .set_host("api.mailjet.com")
            .add_route("v3")
            .add_route("REST")
            .add_route("contact");
        search.add_parameters_to_url(&mut ub);

        let (response, code) = self.get(&ub.build())?;

        let object = serde_json::from_str(&response).ok();
        Ok(Response::create_from_data(code.ok(),response, object))
    }

    /// Retrieve a specific contact
    ///
    /// Includes information about contact status and creation / activity timestamps
    ///
    /// # Parameters
    ///
    /// * `identifier`: The id or the email of the contact
    pub fn contact_search_from_id_or_email(
        &self,
        identifier: &ContactIdentifier,
    ) -> Result<Response<ContactResponse>, Box<dyn StdError>> {
        let mut ub = URLBuilder::new();

        ub.set_protocol("https")
            .set_host("api.mailjet.com")
            .add_route("v3")
            .add_route("REST")
            .add_route("contact")
            .add_route(&identifier.to_string());

        let (response, code) = self.get(&ub.build())?;

        let object = serde_json::from_str(&response).ok();
        Ok(Response::create_from_data(code.ok(),response, object))
    }

    /// Update the user-given name and exclusion status of a specific contact
    ///
    /// # Parameters
    ///
    /// * `identifier`: The id or the email of the contact to update
    /// * `request`: The updated information
    pub fn contact_update(
        &self,
        identifier: &ContactIdentifier,
        request: &ContactRequest,
    ) -> Result<Response<ContactResponse>, Box<dyn StdError>> {
        let j = serde_json::to_string(&request)?;
        let mut ub = URLBuilder::new();

        ub.set_protocol("https")
            .set_host("api.mailjet.com")
            .add_route("v3")
            .add_route("REST")
            .add_route("contact")
            .add_route(&identifier.to_string());

        let (response, code) = self.put(&ub.build(), &j)?;

        let object = serde_json::from_str(&response).ok();
        Ok(Response::create_from_data(code.ok(),response, object))
    }

    /// Delete a contact
    ///
    /// This function works only if you are in a country under GDPR law
    ///
    /// # Parameters
    ///
    /// * `contact_id`: The contact's id
    pub fn contact_delete(&self, contact_id: i128) -> Result<Response<bool>, Box<dyn StdError>> {
        let mut ub = URLBuilder::new();

        ub.set_protocol("https")
            .set_host("api.mailjet.com")
            .add_route("v4")
            .add_route("contacts")
            .add_route(&contact_id.to_string());

        let (response, code) = self.delete(&ub.build())?;

        Ok(Response::create_from_data(code.clone().ok(), response, Some((200..300).contains(&code.unwrap_or_default()))))
    }

    /// Create a new contact list
    ///
    /// # Parameters
    ///
    /// * `request`: The request containing contacts list data
    pub fn contacts_list_create(
        &self,
        request: &ContactsListRequest,
    ) -> Result<ContactsListResponse, Box<dyn StdError>> {
        let j = serde_json::to_string(request)?;
        let (response, _) = self.post("https://api.mailjet.com/v3/REST/contactslist", &j)?;
        Ok(serde_json::from_str(&response)?)
    }

    /// Retrieve details for all contact lists - name, subscriber count,
    /// creation timestamp, deletion status
    ///
    /// # Parameters
    ///
    /// * `search`: The search arguments
    pub fn contacts_list_search(
        &self,
        search: &ContactsListSearchRequest,
    ) -> Result<ContactsListResponse, Box<dyn StdError>> {
        let mut ub = URLBuilder::new();

        ub.set_protocol("https")
            .set_host("api.mailjet.com")
            .add_route("v3")
            .add_route("REST")
            .add_route("contactslist");

        search.add_parameters_to_url(&mut ub);

        let (response, _) = self.get(&ub.build())?;

        Ok(serde_json::from_str(&response)?)
    }

    /// Retrieve details for a specific contact list - name, subscriber count,
    /// creation timestamp, deletion status
    ///
    /// # Parameters
    ///
    /// * `identifier`: The id or the address of the contacts list to retrieve
    pub fn contacts_list_search_from_id_or_address(
        &self,
        identifier: &ContactsListIdentifier,
    ) -> Result<ContactsListResponse, Box<dyn StdError>> {
        let mut ub = URLBuilder::new();

        ub.set_protocol("https")
            .set_host("api.mailjet.com")
            .add_route("v3")
            .add_route("REST")
            .add_route("contactslist")
            .add_route(&identifier.to_string());

        let (response, _) = self.get(&ub.build())?;

        Ok(serde_json::from_str(&response)?)
    }

    /// Update a specific contact list by changing its name and / or deletion status
    ///
    /// # Parameters
    ///
    /// * `identifier`: The id or the email of the contact to update
    /// * `request`: The updated information
    pub fn contacts_list_update(
        &self,
        identifier: &ContactsListIdentifier,
        request: &ContactsListRequest,
    ) -> Result<ContactsListResponse, Box<dyn StdError>> {
        let mut ub = URLBuilder::new();
        let j = serde_json::to_string(request)?;

        ub.set_protocol("https")
            .set_host("api.mailjet.com")
            .add_route("v3")
            .add_route("REST")
            .add_route("contactslist")
            .add_route(&identifier.to_string());

        let (response, _) = self.put(&ub.build(), &j)?;

        Ok(serde_json::from_str(&response)?)
    }

    /// Delete a contact list
    ///
    /// The ContactsList object will continue to exist with Deleted status for
    /// 30 days, and can be reinstated by changing the value of IsDeleted to false
    /// via an update
    ///
    /// # Parameters
    ///
    /// * `identifier`: The id or the address of the contacts list to delete
    pub fn contacts_list_delete(
        &self,
        identifier: &ContactsListIdentifier,
    ) -> Result<bool, Box<dyn StdError>> {
        let mut ub = URLBuilder::new();

        ub.set_protocol("https")
            .set_host("api.mailjet.com")
            .add_route("v3")
            .add_route("REST")
            .add_route("contactslist")
            .add_route(&identifier.to_string());

        let (_, code) = self.delete(&ub.build())?;

        Ok((200..300).contains(&code.unwrap_or_default()))
    }
}

#[cfg(test)]
mod test {
    use crate::data::{ContactIdentifier, ContactsListIdentifier};
    use crate::requests::{
        ContactRequest, ContactSearchRequest, ContactsListRequest, ContactsListSearchRequest,
    };
    use crate::{
        data::{EmailAddress, Message},
        requests::{MessageInformationRequest, MessageRequest, SendRequest},
        Mailjet,
    };
    use rand::Rng;

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

        let response = mailjet.send(&send_request).unwrap().object.unwrap();

        assert_eq!(response.messages.len(), 1);
        assert_eq!(response.messages.first().unwrap().status, "success");
        assert_eq!(response.messages.first().unwrap().to.len(), 1);
        assert_eq!(
            response.messages.first().unwrap().to.first().unwrap().email,
            "john.doe@example.com"
        );
    }

    #[test]
    fn retrieve_all_messages() {
        let mailjet = Mailjet::from_api_keys(
            &std::env::var("MJ_KEY").unwrap(),
            &std::env::var("MJ_SECRET").unwrap(),
        );

        let response = mailjet.message(&MessageRequest::default()).unwrap().object.unwrap();

        assert!(response.count > 0);
        assert_eq!(response.count as usize, response.data.len());
    }

    #[test]
    fn retrieve_message_from_id() {
        let mailjet = Mailjet::from_api_keys(
            &std::env::var("MJ_KEY").unwrap(),
            &std::env::var("MJ_SECRET").unwrap(),
        );

        let response = mailjet
            .message_from_id(std::env::var("MJ_MESSAGE_ID").unwrap().parse().unwrap())
            .unwrap().object.unwrap();

        assert_eq!(response.count, 1);
        assert_eq!(response.data.len(), 1);
    }

    #[test]
    fn retrieve_message_history_from_id() {
        let mailjet = Mailjet::from_api_keys(
            &std::env::var("MJ_KEY").unwrap(),
            &std::env::var("MJ_SECRET").unwrap(),
        );

        let response = mailjet
            .message_history_from_id(std::env::var("MJ_MESSAGE_ID").unwrap().parse().unwrap())
            .unwrap().object.unwrap();

        assert_eq!(response.count, 1);
        assert_eq!(response.data.len(), 1);
    }

    #[test]
    fn retrieve_all_message_information() {
        let mailjet = Mailjet::from_api_keys(
            &std::env::var("MJ_KEY").unwrap(),
            &std::env::var("MJ_SECRET").unwrap(),
        );

        let search = MessageInformationRequest {
            campaign_id: Some(std::env::var("MJ_CAMPAIGN_ID").unwrap().parse().unwrap()),
            ..Default::default()
        };

        let response = mailjet.message_information(&search).unwrap().object.unwrap();

        assert!(response.count > 0);
        assert_eq!(response.count as usize, response.data.len());
    }

    #[test]
    fn retrieve_message_information_from_id() {
        let mailjet = Mailjet::from_api_keys(
            &std::env::var("MJ_KEY").unwrap(),
            &std::env::var("MJ_SECRET").unwrap(),
        );

        let response = mailjet
            .message_information_from_id(std::env::var("MJ_MESSAGE_ID").unwrap().parse().unwrap())
            .unwrap().object.unwrap();

        assert_eq!(response.count, 1);
        assert_eq!(response.data.len(), 1);
    }

    #[test]
    fn contact_create_and_delete() {
        let mailjet = Mailjet::from_api_keys(
            &std::env::var("MJ_KEY").unwrap(),
            &std::env::var("MJ_SECRET").unwrap(),
        );
        let mut rng = rand::thread_rng();
        let email = format!("dummy.{}.{}@gmail.com", rng.gen::<i64>(), rng.gen::<i64>());
        let contact = ContactRequest {
            email: Some(email.clone()),
            ..Default::default()
        };
        let response = mailjet.contact_create(&contact).unwrap().object.unwrap();

        assert_eq!(response.count, 1);
        assert_eq!(response.data[0].email, email);

        // Test delete only if you are in a GDPR country
        if &std::env::var("MJ_CAN_DELETE_CONTACT").unwrap_or_default() == "1" {
            let id = response.data[0].id;
            assert!(mailjet.contact_delete(id).unwrap().object.unwrap());
        }
    }

    #[test]
    fn contact_get_all() {
        let mailjet = Mailjet::from_api_keys(
            &std::env::var("MJ_KEY").unwrap(),
            &std::env::var("MJ_SECRET").unwrap(),
        );
        let response = mailjet
            .contact_search(&ContactSearchRequest::default())
            .unwrap().object.unwrap();

        assert!(response.count > 0);
    }

    #[test]
    fn contact_get_from_id() {
        let mailjet = Mailjet::from_api_keys(
            &std::env::var("MJ_KEY").unwrap(),
            &std::env::var("MJ_SECRET").unwrap(),
        );
        let contact1 = mailjet
            .contact_search_from_id_or_email(&ContactIdentifier::ContactId(
                std::env::var("MJ_CONTACT_ID").unwrap().parse().unwrap(),
            ))
            .unwrap().object.unwrap();
        let contact2 = mailjet
            .contact_search_from_id_or_email(&ContactIdentifier::ContactEmail(
                std::env::var("MJ_CONTACT_EMAIL").unwrap(),
            ))
            .unwrap().object.unwrap();

        assert_eq!(contact1.count, 1);
        assert_eq!(contact2.count, 1);
        assert_eq!(contact1.data[0].id, contact2.data[0].id);
        assert_eq!(contact1.data[0].email, contact2.data[0].email);
    }

    #[test]
    fn contact_update() {
        let mailjet = Mailjet::from_api_keys(
            &std::env::var("MJ_KEY").unwrap(),
            &std::env::var("MJ_SECRET").unwrap(),
        );
        let mut rng = rand::thread_rng();
        let response = mailjet
            .contact_update(
                &ContactIdentifier::ContactEmail(std::env::var("MJ_CONTACT_EMAIL").unwrap()),
                &ContactRequest {
                    name: Some(format!("Camille Nevermind {}", rng.gen::<i64>())),
                    ..Default::default()
                },
            )
            .unwrap().object.unwrap();

        assert_eq!(response.count, 1);
    }

    #[test]
    fn contacts_list_create_and_delete() {
        let mailjet = Mailjet::from_api_keys(
            &std::env::var("MJ_KEY").unwrap(),
            &std::env::var("MJ_SECRET").unwrap(),
        );
        let mut rng = rand::thread_rng();
        let list_name = format!("List {}", rng.gen::<i64>());
        let contact_list = ContactsListRequest {
            name: Some(list_name.clone()),
            ..Default::default()
        };

        let response = mailjet.contacts_list_create(&contact_list).unwrap();

        assert_eq!(response.count, 1);
        assert_eq!(response.data[0].name, list_name);

        let id = response.data[0].id;

        assert!(mailjet
            .contacts_list_delete(&ContactsListIdentifier::ListId(id))
            .unwrap());
    }

    #[test]
    fn contact_list_retrieve_all() {
        let mailjet = Mailjet::from_api_keys(
            &std::env::var("MJ_KEY").unwrap(),
            &std::env::var("MJ_SECRET").unwrap(),
        );
        let response = mailjet
            .contacts_list_search(&ContactsListSearchRequest::default())
            .unwrap();

        assert!(response.count > 0);
    }

    #[test]
    fn contact_list_retrieve_from_identifier() {
        let mailjet = Mailjet::from_api_keys(
            &std::env::var("MJ_KEY").unwrap(),
            &std::env::var("MJ_SECRET").unwrap(),
        );
        let response_id = mailjet
            .contacts_list_search_from_id_or_address(&ContactsListIdentifier::ListId(
                std::env::var("MJ_CONTACTS_LIST_ID")
                    .unwrap()
                    .parse()
                    .unwrap(),
            ))
            .unwrap();

        assert_eq!(response_id.count, 1);
    }

    #[test]
    fn contact_list_update() {
        let mailjet = Mailjet::from_api_keys(
            &std::env::var("MJ_KEY").unwrap(),
            &std::env::var("MJ_SECRET").unwrap(),
        );
        let mut rng = rand::thread_rng();
        let new_name = format!("List {}", rng.gen::<i64>());
        let request = ContactsListRequest {
            name: Some(new_name.clone()),
            ..Default::default()
        };
        let response = mailjet
            .contacts_list_update(
                &ContactsListIdentifier::ListAddress(
                    std::env::var("MJ_CONTACTS_LIST_ID")
                        .unwrap()
                        .parse()
                        .unwrap(),
                ),
                &request,
            )
            .unwrap();

        assert_eq!(response.count, 1);
        assert_eq!(response.data[0].name, new_name);
    }
}
