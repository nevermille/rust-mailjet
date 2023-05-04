# Unoficial wrapper for Mailjet API

Rust wrapper for Mailjet's API

Mailjet is a service provider for sending emails and SMS, visit <https://www.mailjet.com/>
for more information.

**WARNING**: This wrapper is not official, Mailjet won't provide any support for it.

# Send a basic email

```rust
use mailjet_api_wrapper::{
    data::{EmailAddress, Message},
    requests::SendRequest,
    Mailjet,
};

// Create mailjet client
let mailjet = Mailjet::from_api_keys("your_key", "your_secret");

// Create recipients
let to = EmailAddress::from_email("passenger1@mailjet.com");
let from = EmailAddress::from_email_and_name("pilot@mailjet.com", "Mailjet Pilot");

// Create message
let mut message = Message::default();
message.to.push(to);
message.from = from;
message.html_part = "<h3>Dear passenger 1, welcome to <a href=\"https://www.mailjet.com/\">Mailjet</a>!</h3><br />May the delivery force be with you!".to_string();
message.text_part = "Dear passenger 1, welcome to Mailjet! May the delivery force be with you!".to_string();
message.subject = "Your email flight plan!".to_string();

// Create send request
let mut send_request = SendRequest::default();
send_request.sandbox_mode = Some(true); // You can remove this when sending for real
send_request.messages.push(message);

// Send emails
let response = mailjet.send(&send_request).unwrap();
```

# The data structures

The request and response structures are the same as mailjet's JSONs with PascalCase field names
converted into snake_case format as asked by rust. Everything is serializable/deserializable with
serde, so you can easily go back to original JSON formats with serde_json.

For more information on JSON structures, go read <https://dev.mailjet.com/email/guides/send-api-v31/>
