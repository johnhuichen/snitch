use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use log::info;

use crate::config::SMTPInfo;

use super::Snitcher;
use std::error::Error;

pub struct EmailSnitcher {
    smtp_info: SMTPInfo,
}

impl EmailSnitcher {
    pub fn new(smtp_info: SMTPInfo) -> Self {
        EmailSnitcher { smtp_info }
    }
}

impl Snitcher for EmailSnitcher {
    fn snitch(&self, message: String) -> Result<(), Box<dyn Error>> {
        let email = Message::builder()
            .from(
                format!("Sender <{}>", self.smtp_info.smtp_user)
                    .parse()
                    .unwrap(),
            )
            .to(format!("Receiver <{}>", self.smtp_info.recipient)
                .parse()
                .unwrap())
            .subject(self.smtp_info.email_subject.to_owned())
            .body(message)
            .unwrap();

        let creds = Credentials::new(
            self.smtp_info.smtp_user.clone(),
            self.smtp_info.smtp_password.clone(),
        );

        // Open a remote connection to gmail
        let mailer = SmtpTransport::relay(&self.smtp_info.smtp_server)
            .unwrap()
            .credentials(creds)
            .build();

        // Send the email
        match mailer.send(&email) {
            Ok(_) => info!("Email sent successfully!"),
            Err(e) => info!("Could not send email: {:?}", e),
        }
        Ok(())
    }
}
