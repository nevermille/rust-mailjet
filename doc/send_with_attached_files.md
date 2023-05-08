# Send with attached files

```rust
use mailjet_api_wrapper::{
    data::{EmailAddress, Message, Attachment},
    requests::SendRequest,
    Mailjet,
};

// Create mailjet client
let mailjet = Mailjet::from_api_keys("your_key", "your_secret");

// Create recipients
let to = EmailAddress::from_email("passenger1@mailjet.com");
let from = EmailAddress::from_email_and_name("pilot@mailjet.com", "Mailjet Pilot");

// Create attachments
let mut attachment = Attachment {
    content_type: "text/plain".to_string(),
    filename: "test.txt".to_string(),
    base64_content: "VGhpcyBpcyB5b3VyIGF0dGFjaGVkIGZpbGUhISEK".to_string(),
    content_id: None
};

// Create message
let mut message = Message::default();
message.to.push(to);
message.from = from;
message.html_part = "<h3>Dear passenger 1, welcome to <a href=\"https://www.mailjet.com/\">Mailjet</a>!</h3><br />May the delivery force be with you!".to_string();
message.text_part = "Dear passenger 1, welcome to Mailjet! May the delivery force be with you!".to_string();
message.subject = "Your email flight plan!".to_string();
message.attachments.push(attachment);

// Create send request
let mut send_request = SendRequest::default();
send_request.sandbox_mode = Some(true); // You can remove this when sending for real
send_request.messages.push(message);

// Send emails
let response = mailjet.send(&send_request).unwrap_or_default();
```
