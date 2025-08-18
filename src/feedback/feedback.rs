use crate::feedback::{
    BlockedFeedback, BounceFeedback, ClickFeedback, OpenFeedback, SentFeedback, SpamFeedback,
    UnsubFeedback,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The feedback enum for all the possible events
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "event")]
pub enum Feedback {
    /// A sent event
    #[serde(rename = "sent")]
    Sent(SentFeedback),

    /// An open event
    #[serde(rename = "open")]
    Open(OpenFeedback),

    /// A click event
    #[serde(rename = "click")]
    Click(ClickFeedback),

    /// A bounce event
    #[serde(rename = "bounce")]
    Bounce(BounceFeedback),

    /// A blocked event
    #[serde(rename = "blocked")]
    Blocked(BlockedFeedback),

    /// A spam event
    #[serde(rename = "spam")]
    Spam(SpamFeedback),

    /// An unsub event
    #[serde(rename = "unsub")]
    Unsub(UnsubFeedback),

    /// The event could not be deserialized and was not JSON compatible
    Invalid(String),

    /// The event could not be deserialized but was JSON compatible
    #[serde(untagged)]
    Unknown(GenericFeedback),
}

impl From<&str> for Feedback {
    fn from(value: &str) -> Self {
        let json_value = serde_json::from_str::<Feedback>(value);

        if json_value.is_err() {
            return Feedback::Invalid(value.to_string());
        }
        json_value.unwrap()
    }
}

impl From<String> for Feedback {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}

impl From<&String> for Feedback {
    fn from(value: &String) -> Self {
        value.as_str().into()
    }
}

impl From<&Value> for Feedback {
    fn from(value: &Value) -> Self {
        let feedback = serde_json::from_value(value.clone());

        if feedback.is_err() {
            return Feedback::Invalid(value.to_string());
        }
        feedback.unwrap()
    }
}

impl From<Value> for Feedback {
    fn from(value: Value) -> Self {
        (&value).into()
    }
}

/// A generic event feedback
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GenericFeedback {
    /// Unix timestamp of event
    #[serde(default)]
    time: i64,

    /// The unique message ID
    #[serde(default)]
    #[serde(rename = "MessageID")]
    message_id: i64,

    /// The unique message GUID
    #[serde(default)]
    #[serde(rename = "Message_GUID")]
    message_guid: String,

    /// Email address of recipient triggering the event
    #[serde(default)]
    email: String,

    /// Internal Mailjet campaign ID associated to the message
    #[serde(default)]
    mj_campaign_id: i64,

    /// Internal Mailjet contact ID
    #[serde(default)]
    mj_contact_id: i64,

    /// Value of the X-Mailjet-Campaign header when provided
    #[serde(default)]
    #[serde(rename = "customcampaign")]
    custom_campaign: String,

    /// The custom ID, when provided at send time
    #[serde(default)]
    #[serde(rename = "CustomID")]
    custom_id: String,

    /// The event payload, when provided at send time
    #[serde(default)]
    #[serde(rename = "Payload")]
    payload: String,
}

#[cfg(test)]
mod tests {
    use crate::feedback::Feedback;

    #[test]
    fn serialize_sent() {
        let feedback_str = r#"{"event":"sent","time":1433333949,"MessageID":19421777835146490,"Message_GUID":"1ab23cd4-e567-8901-2345-6789f0gh1i2j","email":"api@mailjet.com","mj_campaign_id":7257,"mj_contact_id":4,"customcampaign":"","mj_message_id":"19421777835146490","smtp_reply":"sent (250 2.0.0 OK 1433333948 fa5si855896wjc.199 - gsmtp)","CustomID":"helloworld","Payload":""}"#;
        let feedback = Feedback::from(feedback_str);

        assert!(matches!(feedback, Feedback::Sent(_)));
    }

    #[test]
    fn serialize_open() {
        let feedback_str = r#"{"event":"open","time":1433103519,"MessageID":19421777396190490,"Message_GUID":"1ab23cd4-e567-8901-2345-6789f0gh1i2j","email":"api@mailjet.com","mj_campaign_id":7173,"mj_contact_id":320,"customcampaign":"","CustomID":"helloworld","Payload":"","ip":"127.0.0.1","geo":"US","agent":"Mozilla/5.0 (Windows NT 5.1; rv:11.0) Gecko Firefox/11.0"}"#;
        let feedback = Feedback::from(feedback_str);

        assert!(matches!(feedback, Feedback::Open(_)));
    }

    #[test]
    fn serialize_click() {
        let feedback_str = r#"{"event":"click","time":1433334653,"MessageID":19421777836302490,"Message_GUID":"1ab23cd4-e567-8901-2345-6789f0gh1i2j","email":"api@mailjet.com","mj_campaign_id":7272,"mj_contact_id":4,"customcampaign":"","CustomID":"helloworld","Payload":"","url":"https://mailjet.com","ip":"127.0.0.1","geo":"FR","agent":"Mozilla/5.0 (Macintosh; Intel Mac OS X 10_10_0) AppleWebKit/537.36"}"#;
        let feedback = Feedback::from(feedback_str);

        assert!(matches!(feedback, Feedback::Click(_)));
    }

    #[test]
    fn serialize_bounce() {
        let feedback_str = r#"{"event":"bounce","time":1430812195,"MessageID":13792286917004336,"Message_GUID":"1ab23cd4-e567-8901-2345-6789f0gh1i2j","email":"bounce@mailjet.com","mj_campaign_id":0,"mj_contact_id":0,"customcampaign":"","CustomID":"helloworld","Payload":"","blocked":false,"hard_bounce":true,"error_related_to":"recipient","error":"user unknown","comment":"Host or domain name not found. Name service error for name=lbjsnrftlsiuvbsren.com type=A: Host not found"}"#;
        let feedback = Feedback::from(feedback_str);

        assert!(matches!(feedback, Feedback::Bounce(_)));
    }

    #[test]
    fn serialize_blocked() {
        let feedback_str = r#"{"event":"blocked","time":1430812195,"MessageID":13792286917004336,"Message_GUID":"1ab23cd4-e567-8901-2345-6789f0gh1i2j","email":"bounce@mailjet.com","mj_campaign_id":0,"mj_contact_id":0,"customcampaign":"","CustomID":"helloworld","Payload":"","error_related_to":"recipient","error":"user unknown"}"#;
        let feedback = Feedback::from(feedback_str);

        assert!(matches!(feedback, Feedback::Blocked(_)));
    }

    #[test]
    fn serialize_spam() {
        let feedback_str = r#"{"event":"spam","time":1430812195,"MessageID":13792286917004336,"Message_GUID":"1ab23cd4-e567-8901-2345-6789f0gh1i2j","email":"bounce@mailjet.com","mj_campaign_id":0,"mj_contact_id":0,"customcampaign":"","CustomID":"helloworld","Payload":"","source":"JMRPP"}"#;
        let feedback = Feedback::from(feedback_str);

        assert!(matches!(feedback, Feedback::Spam(_)));
    }

    #[test]
    fn serialize_unsub() {
        let feedback_str = r#"{"event":"unsub","time":1433334941,"MessageID":20547674933128000,"Message_GUID":"1ab23cd4-e567-8901-2345-6789f0gh1i2j","email":"api@mailjet.com","mj_campaign_id":7276,"mj_contact_id":126,"customcampaign":"","CustomID":"helloworld","Payload":"","mj_list_id":1,"ip":"127.0.0.1","geo":"FR","agent":"Mozilla/5.0 (Macintosh; Intel Mac OS X 10_10_0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/42.0.2311.135 Safari/537.36"}"#;
        let feedback = Feedback::from(feedback_str);

        assert!(matches!(feedback, Feedback::Unsub(_)));
    }
}
