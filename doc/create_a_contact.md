# Create a contact

```rust
use mailjet_api_wrapper::requests::ContactRequest;
use mailjet_api_wrapper::Mailjet;

// Create mailjet client
let mailjet = Mailjet::from_api_keys("your_key", "your_secret");

/// Create new contact
let contact = ContactRequest {
    is_excluded_from_campaigns: Some(true),
    name: Some("New Contact".to_string()),
    email: Some("passenger@mailjet.com".to_string()),
};

/// Execution
let response = mailjet.contact_create(&contact).unwrap_or_default();
```
