[![Building](https://github.com/nevermille/rust-mailjet/actions/workflows/test-stable.yml/badge.svg)](https://github.com/nevermille/rust-mailjet/actions/workflows/test-stable.yml)

# Unoficial wrapper for Mailjet API

Rust wrapper for Mailjet's API

Mailjet is a service provider for sending emails and SMS, visit <https://www.mailjet.com/>
for more information.

**WARNING**: This wrapper is not official, Mailjet won't provide any support for it.

# Getting Started

The first step is to instantiate a Mailjet structure with your API keys:

```rust
use mailjet_api_wrapper::Mailjet;

let mailjet = Mailjet::from_api_keys("your_api_key", "your_api_secret");
```

Now, each route in the API correspond to a function in your mailjet object, for example with message route:

```rust
use mailjet_api_wrapper::Mailjet;
use mailjet_api_wrapper::requests::MessageRequest;

let mailjet = Mailjet::from_api_keys("your_api_key", "your_api_secret");
let res = mailjet.message(&MessageRequest::default()).unwrap_or_default();
```

Those functions have an explicit name, you'll find them easily in the complete documentation or directly in your IDE.

# Simple routes

Those routes only have a few variables in the url and have no parameters, they appear in the API reference like `/message/{message_ID}`. In this case, the function name will be suffixed by `_from_id`.

```rust
use mailjet_api_wrapper::Mailjet;
use mailjet_api_wrapper::requests::MessageRequest;

let mailjet = Mailjet::from_api_keys("your_api_key", "your_api_secret");
let res = mailjet.message_from_id(123456789).unwrap_or_default();
```

# Complex routes

Those routes have parameters to append at the end of the URL like `/message` or `/send`. In this case, you need to build a request object before passing it to the function. You can find them in the `requests` module.

```rust
use mailjet_api_wrapper::Mailjet;
use mailjet_api_wrapper::requests::MessageInformationRequest;

let mailjet = Mailjet::from_api_keys("your_api_key", "your_api_secret");

let request = MessageInformationRequest {
    campaign_id: Some(544678796), // Corresponds to parameter CampaignID
    ..Default::default()
};

let res = mailjet.message_information(&request).unwrap_or_default();
```

# Response

Every route returning a JSON will build a response object. You can find them in the `responses` module.

```rust
use mailjet_api_wrapper::Mailjet;
use mailjet_api_wrapper::requests::MessageRequest;

let mailjet = Mailjet::from_api_keys("your_api_key", "your_api_secret");
let res = mailjet.message(&MessageRequest::default()).unwrap_or_default();

let count = res.count; // Corresponds to Count response field
let data = res.data; // Corresponds to Data response field
```

# The data structures

The request and response structures are the same as mailjet's JSONs and parameters' names with PascalCase field names
converted into snake_case format as asked by rust. Every JSON request and response is serializable/deserializable with
serde, so you can easily build objects with JSON formats with serde_json.

For complete information on JSON structures and parameters, go read <https://dev.mailjet.com/email/reference/>

# Example: send a basic email

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
let response = mailjet.send(&send_request).unwrap_or_default();
```

# Features

| Categories | Features | Actions |
|------------|----------|---------|
| Send Emails | - | ✅ Send an email message via Send API v3.1 <br/> ❌ Send an email message via Send API v3 |
| Messages | - | ✅ Get detailed information on all processed messages <br/> ✅ Get detailed information on a specific processed message <br/> ✅ Get the event history for a specific message <br/> ✅ Retrieve sending / size / spam information about all messages <br/> ✅ Retrieve sending / size / spam information about a specific message ID |
| Contact | Contact | ❌ Create a new contact <br/> ❌ Get a list of all contacts <br/> ❌ Get a specific contact <br/> ❌ Update a specific contact |
| Contact | Contact List | ❌ Create a new contact list <br/> ❌ Retrieve general details for all contact lists <br/> ❌ Retrieve information on a specific contact list <br/> ❌ Update a specific contact list <br/> ❌ Delete a contact list |
| Contact | Bulk Contact Management | ❌ Add, remove or unsubscribe a list of contacts to/from a selection of contact lists <br/> ❌ Add, unsubscribe or remove the contacts present in a list to / from another list <br/> ❌ Add, remove or unsubscribe a list of contacts to/from a specific contact list <br/> ❌ Use an uploaded CSV list of contacts to manage their presence and subscription status in a contact list <br/> ❌ Monitor a submitted upload / update request for multiple contacts <br/> ❌ Monitor a submitted upload / update request for contacts from one list to another <br/> ❌ Monitor a submitted upload / update request for multiple contacts into a specific contact list <br/> ❌ View the status of a CSV import job <br/> ❌ Update or abort a contact CSV import job in progress |
| Contact | Contact Properties | ❌ Create a new contact property <br/> ❌ Get information on all contacts and the property values associated with them <br/> ❌ Get the contact property values relating to a specific contact <br/> ❌ Get information on all contact properties <br/> ❌ Get information on a specific contact property <br/> ❌ Update the contact property values relating to a specific contact <br/> ❌ Update the settings of an existing contact property <br/> ❌ Delete all contact property values relating to a specific contact <br/> ❌ Delete an existing contact property |
| Contact | Subscriptions | ❌ Add, remove or unsubscribe a contact to/from a selection of contact lists <br/> ❌ Manage a single contact subscription to a specific contact list <br/> ❌ Create a new list recipient <br/> ❌ Get all contact lists for a specific contact <br/> ❌ Get info on all signup requests via a subscription widget <br/> ❌ Get info on a specific signup request via a subscription widget <br/> ❌ Get details on all list recipients <br/> ❌ Get details for a specific list recipient <br/> ❌ Update the subscription status of a list recipient <br/> ❌ Delete a list recipient |
| Contact | Verifications | ❌ Start a contact list verification <br/> ❌ Retrieves the current state of the contact list verification job |
| Campaigns | Drafts | ❌ Create a new campaign draft <br/> ❌ Manage the content of your campaign draft email <br/> ❌ Get all campaign drafts and their configuration <br/> ❌ Get a specific campaign draft and its configuration details <br/> ❌ Retrieve the content of your campaign draft email <br/> ❌ Update an existing campaign draft <br/> ❌ Retrieve the sending schedule of a campaign draft <br/> ❌  Schedule the sending of a campaign draft<br/> ❌ Update the sending schedule of a campaign draft <br/> ❌ Cancel the scheduled sending of a campaign draft <br/> ❌ Send a campaign draft immediately <br/> ❌ View the sending status of a campaign draft <br/> ❌ Send a test email for a specified campaign draft |
| Campaigns | Sent Campaigns | ❌ Get detailed information about all campaigns <br/> ❌ Get detailed information about a specific campaign <br/> ❌ Delete a campaign or mark it as starred |
| Segmentation | - | ❌ Create a new contact segmentation formula <br/> ❌ Get a list of all existing segments <br/> ❌ Get an existing contact segmentation formula <br/> ❌ Update an existing contact segmentation formula <br/> ❌ Delete an existing contact segmentation formula |
| Templates | - | ❌ Create an email template <br/> ❌ Create the contents of an email template <br/> ❌ Get all email templates <br/> ❌ Retrieve the configuration settings for a specific template <br/> ❌ Get the contents of an email template <br/> ❌ Update the configuration settings of an email template <br/> ❌ Update the contents of an email template <br/> ❌ Delete an email template |
| Statistics | - | ❌ Get general details and stats for all drafts, AB Testing objects and/or sent campaigns <br/> ❌ Get information about a specific draft, AB Testing object or sent campaign <br/> ❌ Get aggregated statistics, grouped by contact <br/> ❌ Get aggregated statistics for a specific contact <br/> ❌ Retrieve aggregated open and click statistics, grouped by recipient country <br/> ❌ Get aggregated statistics, grouped by list recipient <br/> ❌ Get aggregated statistics for a specific list recipient <br/> ❌ Get message-based or event-based aggregated statistics for a specific campaign, contact list, API Key or sender email address <br/> ❌ Get aggregated statistics for all clicked links in a campaign <br/> ❌ Retrieve statistics, aggregated by recipient's Email Service Provider (ESP) <br/> ❌ Get aggregated statistics for all clicked links <br/> ❌ Get open or click counts, grouped by web browser / email client |
| Message Events | - | ❌ Get a list of all bounce events <br/> ❌ Get details for a specific bounce event <br/> ❌ Get a list of all click events <br/> ❌ Get a list of all open events <br/> ❌ Retrieve open event details for a specific message |
| Webhook | - | ❌ Add a new callback URL <br/> ❌ Get a list of all callback URLs <br/> ❌ Get the configuration of a specific callback URL <br/> ❌ Update the configuration of an existing callback URL <br/> ❌ Delete an existing callback URL |
| Parse | - | ❌ Create a new parseroute instance <br/> ❌ Get a list of all parseroute instances <br/> ❌ Get the configuration details for a specific parseroute resource <br/> ❌ Update an existing parseroute instance <br/> ❌ Delete an existing parseroute instance |
| Sender Addresses and Domains | Sender | ❌ Create a new sender email address or domain <br/> ❌ Get a list of all existing sender email addresses and domains <br/> ❌ Retrieve details on a specific sender email address or domain <br/> ❌ Update an existing sender email address or domain <br/> ❌ Delete an existing sender email address or domain <br/> ❌ Validate a sender email address or domain |
| Sender Addresses and Domains | Metasender | ❌ Create a new metasender <br/> ❌ Get a list of all metasenders <br/> ❌ Get a specific metasender <br/> ❌ Update and existing metasender |
| Sender Addresses and Domains | DNS | ❌ Get the SPF and DKIM settings for all sender domains <br/> ❌ Get the SPF and DKIM settings for a specific sender domain <br/> ❌ Perform a DNS validation of a sender domain |
| Settings | API Key Configuration | ❌ Create a new sub-account API Key <br/> ❌ Get all API Keys and their configuration settings <br/> ❌ Get the configuration settings of a specific API Key <br/> ❌ Update an existing API Key |
| Settings | Account Settings | ❌ Retrieve your profile information <br/> ❌ Get general information on your user settings and activity <br/> ❌ Update your profile information <br/> ❌ Update the settings of your User ID |
| Send SMS | - | ❌ Send an SMS Message |
| SMS Messages | - | ❌ Request an export of SMS messages <br/> ❌ Retrieve a list of SMS messages <br/> ❌ Retrieve SMS messages count <br/> ❌ Retrieve an export request result <br/> ❌ Retrieve a single SMS message |
