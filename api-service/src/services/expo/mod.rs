use expo_push_notification_client::{
    CustomError, Expo, ExpoClientOptions, ExpoPushMessage, ExpoPushTicket, ValidationError,
};
use serde::Serialize;

pub enum NotifError {
    ValidationError(ValidationError),
    CustomError(CustomError),
}

pub fn get_expo_client() -> Expo {
    

    Expo::new(ExpoClientOptions::default())
}

pub async fn send_push_notification<T>(
    client: &Expo,
    tokens: Vec<String>,
    title: String,
    body: String,
    data: &T,
) -> Result<Vec<ExpoPushTicket>, NotifError>
where
    T: Serialize,
{
    let message = ExpoPushMessage::builder(tokens)
        .title(title)
        .body(body)
        .data(data)
        .map_err(NotifError::ValidationError)?
        .sound("default")
        .build()
        .map_err(NotifError::ValidationError)?;

    println!("Sending push notification: {:?}", message);

    let push_tickets = client
        .send_push_notifications(message)
        .await
        .map_err(NotifError::CustomError)?;

    Ok(push_tickets)
}
