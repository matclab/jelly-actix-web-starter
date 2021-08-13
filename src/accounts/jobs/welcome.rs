use std::future::Future;
use std::pin::Pin;

use jelly::anyhow::{anyhow, Error};
use jelly::email::Email;
use jelly::jobs::{Job, JobState, DEFAULT_QUEUE};
use jelly::serde::{Deserialize, Serialize};
use jelly::tera::Context;

use crate::accounts::Account;

/// A job for sending a Welcome email, generally dispatched after an account
/// has been verified.
#[derive(Debug, Serialize, Deserialize)]
pub struct SendWelcomeAccountEmail {
    pub to: i32,
}

impl Job for SendWelcomeAccountEmail {
    type State = JobState;
    type Future = Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;

    const NAME: &'static str = "SendWelcomeAccountEmailJob";
    const QUEUE: &'static str = DEFAULT_QUEUE;

    fn run(self, state: JobState) -> Self::Future {
        Box::pin(async move {
            let (name, email) = Account::fetch_email(self.to, &state.pool)
                .await
                .map_err(|e| anyhow!("Error fetching user name/email: {:?}", e))?;

            let email = Email::new(
                "email/welcome",
                &[email],
                "Welcome to the service",
                {
                    let mut context = Context::new();
                    context.insert("name", &name);
                    context
                },
                state.templates,
            );

            email?.send()?;

            Ok(())
        })
    }
}
