use crate::errors::ServiceError;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

#[tracing::instrument]
pub fn get_smtp_creds() -> Credentials {
    let smtp_username = std::env::var("SMTP_USERNAME").expect("SMTP_USERNAME should be set");
    let smtp_password = std::env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD should be set");

    Credentials::new(smtp_username.to_owned(), smtp_password.to_owned())
}

#[tracing::instrument]
pub fn send_email(html_email_body: String, to_address: String) -> Result<(), ServiceError> {
    let smtp_relay = std::env::var("SMTP_RELAY").expect("SMTP_RELAY should be set");
    let smtp_email_address =
        std::env::var("SMTP_EMAIL_ADDRESS").expect("SMTP_EMAIL_ADDRESS should be set");

    let smtp_creds = get_smtp_creds();
    let mailer = SmtpTransport::relay(smtp_relay.as_str())
        .expect("Failed to create mailer")
        .credentials(smtp_creds)
        .build();

    let email = Message::builder()
        .from(smtp_email_address.parse().expect("Invalid email address"))
        .to(to_address.parse().expect("Invalid email address"))
        .subject("Trieve Sign Up Invitation")
        .header(ContentType::TEXT_HTML)
        .body(html_email_body)
        .expect("Failed to create email");

    match mailer.send(&email) {
        Ok(_) => Ok(()),
        Err(e) => {
            log::error!("Error sending email: {:?}", e);
            Err(ServiceError::BadRequest("Error sending email.".to_string()))
        }
    }
}
