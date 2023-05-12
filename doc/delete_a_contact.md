# GDPR Delete contacts

```rust
use mailjet_api_wrapper::Mailjet;

// Create mailjet client
let mailjet = Mailjet::from_api_keys("your_key", "your_secret");

// Delete contact from id
mailjet.contact_delete(187464256).unwrap_or_default();
```
