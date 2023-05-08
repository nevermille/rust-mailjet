# Set global payload properties

```rust
use mailjet_api_wrapper::{
    data::{EmailAddress, Message},
    requests::SendRequest,
    Mailjet,
};
use serde_json::json;

// Create mailjet client
let mailjet = Mailjet::from_api_keys("your_key", "your_secret");

// Create recipients
let from = EmailAddress::from_email_and_name("pilot@mailjet.com", "Mailjet Pilot");
let to1 = EmailAddress::from_email("passenger1@mailjet.com");
let to2 = EmailAddress::from_email("passenger2@mailjet.com");

// Create message 1
let mut message1 = Message::default();
message1.to.push(to1);
message1.html_part = "<h3>Dear passenger 1, welcome to <a href=\"https://www.mailjet.com/\">Mailjet</a>!</h3><br />May the delivery force be with you!".to_string();
message1.text_part = "Dear passenger 1, welcome to Mailjet! May the delivery force be with you!".to_string();
message1.subject = "Your email flight plan!".to_string();

// Create message 1
let mut message2 = Message::default();
message2.to.push(to2);
message2.html_part = "<h3>Dear passenger 2, welcome to <a href=\"https://www.mailjet.com/\">Mailjet</a>!</h3><br />May the delivery force be with you!".to_string();
message2.text_part = "Dear passenger 2, welcome to Mailjet! May the delivery force be with you!".to_string();
message2.subject = "Your email flight plan!".to_string();

// Create send request
let mut send_request = SendRequest::default();
send_request.sandbox_mode = Some(true); // You can remove this when sending for real
send_request.globals = Some(json!({"From": from}));
send_request.messages.push(message1);
send_request.messages.push(message2);

// Send emails
let response = mailjet.send(&send_request).unwrap();
```
