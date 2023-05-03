use crate::{requests::SendRequest, responses::SendResponse};
use curl::{
    easy::{Easy, List},
    Error,
};
use std::{error::Error as StdError, io::Read};

pub struct Mailjet {
    pub api_key: String,
    pub api_secret: String,
}

impl Mailjet {
    pub fn from_api_keys(key: &str, secret: &str) -> Self {
        Self {
            api_key: key.to_string(),
            api_secret: secret.to_string(),
        }
    }

    fn exec(&self, url: &str, data: &str, post: bool) -> Result<String, Error> {
        let mut curl = Easy::new();
        let mut response: Vec<u8> = Vec::new();

        let data_json_string = data.to_string();
        let mut raw_data = data_json_string.as_str().as_bytes();

        curl.url(url)?;
        curl.username(self.api_key.as_str())?;
        curl.password(self.api_secret.as_str())?;
        curl.post(post)?;

        let mut header_list = List::new();
        header_list.append("Content-Type: application/json")?;
        curl.http_headers(header_list)?;

        {
            let mut transfer = curl.transfer();

            transfer.read_function(|buffer| Ok(raw_data.read(buffer).unwrap_or_default()))?;

            transfer.write_function(|buffer| {
                let _ = &response.extend_from_slice(buffer);
                Ok(buffer.len())
            })?;

            transfer.perform()?;
        }

        Ok(String::from_utf8_lossy(&response).to_string())
    }

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
        let mut message = Message::default();
        let mut to = EmailAddress::default();
        let mut from = EmailAddress::default();
        let mailjet = Mailjet::from_api_keys(
            &std::env::var("MJ_KEY").unwrap(),
            &std::env::var("MJ_SECRET").unwrap(),
        );

        to.email = "john.doe@example.com".to_string();
        to.name = "John Doe".to_string();

        from.email = "jane.doe@example.com".to_string();
        from.name = "Jane Doe".to_string();

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
    }
}
