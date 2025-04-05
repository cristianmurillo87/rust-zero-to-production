use super::SubscriberEmail;
use super::SubscriberName;

#[derive(serde::Deserialize)]
pub struct Subscriber {
    pub email: SubscriberEmail,
    pub name: SubscriberName,
}
