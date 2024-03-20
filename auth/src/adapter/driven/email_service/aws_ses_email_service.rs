// use aws_config::meta::region::RegionProviderChain;
// use aws_sdk_sesv2::operation::get_contact_list;
// use aws_sdk_sesv2::{config::Region, meta::PKG_VERSION};
use async_trait::async_trait;
use aws_sdk_sesv2::error::SdkError;
use aws_sdk_sesv2::operation::send_email::{SendEmailOutput, SendEmailError};
use aws_sdk_sesv2::types::{Body, Content, Destination, EmailContent, Message};
use aws_sdk_sesv2::Client;

use crate::application::port::driven::email_service::{EmailServiceTrait, EmailSendError};


pub struct AWSEmailService();

const FROM_ADDRESS: &'static str = "no-reply@geduardo.com";

#[async_trait]
impl EmailServiceTrait<Client> for AWSEmailService {
    async fn send_confirmation_email(&self, conn: &Client, address: String, code: String) -> Result<(), EmailSendError> {
        
        let contact_list = address.clone();

        match send_message(
            &conn, 
            &contact_list, 
            FROM_ADDRESS, 
            "Comfirm your email", 
            &format!("Code: {}", code)
        ).await {
            Ok(_) => Ok(()),
            Err(err) => Err(EmailSendError::Unknown(err.to_string())),
        }
    }

    async fn send_reset_password_email(&self, conn: &Client, address: String, link: String) -> Result<(), EmailSendError> {

        let contact_list = address.clone();

        match send_message(
            &conn, 
            &contact_list, 
            FROM_ADDRESS, 
            "Reset your password", 
            &format!("Link: {}", link)
        ).await {
            Ok(_) => Ok(()),
            Err(err) => Err(EmailSendError::Unknown(err.to_string())),
        }
    }
}

// Sends a message to all members of the contact list.
// snippet-start:[ses.rust.send-email]
async fn send_message(
    client: &Client,
    recipient: &str,
    from: &str,
    subject: &str,
    message: &str,
) -> Result<SendEmailOutput, SdkError<SendEmailError>> {
    let mut dest: Destination = Destination::builder().build();
    dest.to_addresses = Some(vec![String::from(recipient)]);
    let subject_content = Content::builder()
        .data(subject)
        .charset("UTF-8")
        .build()
        .expect("building Content");
    let body_content = Content::builder()
        .data(message)
        .charset("UTF-8")
        .build()
        .expect("building Content");
    let body = Body::builder().text(body_content).build();

    let msg = Message::builder()
        .subject(subject_content)
        .body(body)
        .build();

    let email_content = EmailContent::builder().simple(msg).build();

    client
        .send_email()
        .from_email_address(from)
        .destination(dest)
        .content(email_content)
        .send()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use aws_config::{meta::region::RegionProviderChain, Region};


    #[tokio::test]
    async fn test_send_confirmation_email() {
        let region_provider = RegionProviderChain::first_try(Region::new("us-east-2"))
            .or_default_provider();

        let shared_config = aws_config::from_env().region(region_provider).load().await;

        let client = Client::new(&shared_config);

        let email_service = AWSEmailService();

        let address = String::from("felover496@evvgo.com");
        let code = String::from("123456");

        let result = email_service.send_confirmation_email(&client, address, code).await;
        assert!(result.is_ok(), "{:?}", result.err());
    }
}
