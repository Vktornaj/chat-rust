use lettre::message::header::ContentType;
use lettre::{Message, SmtpTransport, Transport};
use async_trait::async_trait;

use crate::application::port::driven::email_servic::{EmailServiceTrait, EmailSendError};

struct FakeEmailService();

#[async_trait]
impl EmailServiceTrait<SmtpTransport> for FakeEmailService {
    async fn send_confirmation_email(&self, conn: &SmtpTransport, address: String, code: u32) -> Result<(), EmailSendError> {
        let email = Message::builder()
            .from("Staff <staff@domain.tld>".parse().unwrap())
            .reply_to("You <you@domain.tld>".parse().unwrap())
            .to(format!("You <{}>", address).parse().unwrap())
            .subject(String::from("Confirm your email"))
            .header(ContentType::TEXT_PLAIN)
            .body(format!("Code: {}", code))
            .unwrap();

        // Send the email
        match conn.send(&email) {
            Ok(_) => Ok(()),
            Err(e) => panic!("Could not send email: {e:?}"),
        }
    }
}

// write tests
#[cfg(test)]
mod tests {
    use rocket::{tokio, futures::TryStreamExt};
    use async_imap::Client;
    use tokio::net::TcpStream;

    use crate::domain::types::code::Code;

    use super::*;

    #[tokio::test]
    async fn test_send_confirmation_email() {
        // Send email
        let email_service = FakeEmailService();
        let mailer: SmtpTransport = SmtpTransport::unencrypted_localhost();

        let res = mailer.test_connection();
        assert!(res.is_ok(), "{}", res.err().unwrap());

        let address = String::from("you@domain.tld");
        let code: u32 = Code::new(6).into();

        let res = email_service.send_confirmation_email(&mailer, address, code).await;
        assert!(res.is_ok());

        // sleep for 1 second to allow email to be sent
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        // retrieve email
        let tls_stream = TcpStream::connect("localhost:143").await.unwrap();
        
        let client = Client::new(tls_stream);

        let mut session = client
            .login("", "").await
            .map_err(|(err, _client)| err).unwrap();

        session.select("INBOX").await.unwrap();

        // Fetch all messages in this mailbox, along with its RFC 822 field.
        // RFC 822 dictates the format of the body of e-mails.
        let messages_stream = session.fetch("1:*", "RFC822").await.unwrap();
        let messages: Vec<_> = messages_stream.try_collect().await.unwrap();
        // let message = messages.first().expect("found no messages in the INBOX");

        let bodyes = messages.iter().map(|m| {
            // Extract the message body.
            let body = m.body().expect("message did not have a body!");
            std::str::from_utf8(body).expect("message was not valid utf-8").to_string()
        }).collect::<Vec<String>>();

        let matches_bodyes: Vec<String> = bodyes.into_iter()
            .filter(|b| b.contains(format!("Code: {}", code).as_str())).collect();

        assert_eq!(matches_bodyes.len(), 1);

        session.logout().await.unwrap();
    }
}