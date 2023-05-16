# Create a contact list

```rust
use mailjet_api_wrapper::Mailjet;
use mailjet_api_wrapper::requests::ContactsListRequest;

// Create mailjet client
let mailjet = Mailjet::from_api_keys("your_key", "your_secret");

/// Create new contact list
let contact_list = ContactsListRequest {
    name: Some("my_contactslist".to_string()),
    ..Default::default()
};

/// Execution
let response = mailjet.contacts_list_create(&contact_list).unwrap_or_default();
```
