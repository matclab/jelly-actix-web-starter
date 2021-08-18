use std::env::var;

use anyhow::Result;

use super::common::env_exists_and_not_empty;
pub use super::common::Email;

use sendgrid::v3::*;
use lettre::message::Mailbox;
use std::str::FromStr;


/// Check that all needed environment variables are set and not empty.
pub fn check_conf() {
    vec![
        "SENDGRID_API_KEY",
    ]
    .into_iter()
    .for_each(|env| env_exists_and_not_empty(env));
}

impl Email {
    /// Send the email. Relies on you ensuring that `EMAIL_DEFAULT_FROM`,
    /// `EMAIL_SMTP_HOST`, `EMAIL_SMTP_USERNAME`, and `EMAIL_SMTP_PASSWORD`
    /// are set in your `.env`.
    pub fn send_via_sendgrid(&self) -> Result<(), anyhow::Error> {

        // Build a sendgrid addres with the help of letter Mailbox (not so clean indeed).
        let mailbox = Mailbox::from_str(&self.to)?;
        let mut to_address = sendgrid::v3::Email::new(mailbox.email.to_string());
        if let Some(name) = mailbox.name {
            to_address = to_address.set_name(name);
        }
        let message = Message::new(to_address)
            .set_subject(&self.subject)
            .add_content(
                Content::new()
                    .set_content_type("text/html")
                    .set_value(&self.bodyhtml)
            )
            .add_content(
                Content::new()
                    .set_content_type("text/plain")
                    .set_value(&self.body)
            );

        let api_key = var("SENDGRID_API_KEY").expect("SENDGRID_API_KEY not set!");
        let sender = Sender::new(api_key);
        sender.send(&message)?;
        Ok(())
    }
}
