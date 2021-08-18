//! Provides a very, very basic `Email` struct that can send via Postmark.
//! This is designed for transactional emails - if you need otherwise,
//! you're free to import Lettre or whatever.
//!
//! If you prefer a different provider than Postmark, you can swap the
//! send implementation in here.
use std::env::var;

use super::common::{Email,env_exists_and_not_empty};

/// Check that all needed environment variables are set and not empty.
pub fn check_conf() {
    vec![
        "POSTMARK_API_KEY",
    ]
    .into_iter()
    .for_each(|env| env_exists_and_not_empty(env));
}


impl Email {

    /// Send the email. Relies on you ensuring that `POSTMARK_API_KEY`
    /// is set in your `.env`.
    pub fn send_via_postmark(&self) -> Result<(), anyhow::Error> {
        let api_key = var("POSTMARK_API_KEY")
            .expect("POSTMARK_API_KEY not set!");

        minreq::post("https://api.postmarkapp.com/email")
            .with_header("X-Postmark-Server-Token", api_key)
            .with_json(&self)?.send()?;

        Ok(())
    }
}

