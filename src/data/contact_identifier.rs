/// The contact identifier, can be an id or an email address
pub enum ContactIdentifier {
    /// Unique numeric ID of the contact you want to retrieve
    ContactId(i128),
    /// The email address of the contact your want to retrieve
    ContactEmail(String),
}
