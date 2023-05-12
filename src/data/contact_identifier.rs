/// The contact identifier, can be an id or an email address
pub enum ContactIdentifier {
    /// Unique numeric ID of the contact you want to retrieve
    ContactId(i128),
    /// The email address of the contact your want to retrieve
    ContactEmail(String),
}

impl ToString for ContactIdentifier {
    fn to_string(&self) -> String {
        match self {
            ContactIdentifier::ContactId(v) => v.to_string(),
            ContactIdentifier::ContactEmail(v) => v.to_string(),
        }
    }
}
