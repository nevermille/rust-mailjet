# Retrieve a Contact

```rust
use mailjet_api_wrapper::Mailjet;
use mailjet_api_wrapper::data::ContactIdentifier;

// Create mailjet client
let mailjet = Mailjet::from_api_keys("your_key", "your_secret");

// Delete contact from id
let response = mailjet
    .contact_search_from_id_or_email(&ContactIdentifier::ContactEmail("passenger@mailjet.com".to_string()))
    .unwrap_or_default();
```
