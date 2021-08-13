use log::info;
use std::env::var;
use std::env;
use std::sync::{Arc, RwLock};
use tera::{Tera, Context};

use chrono::{Datelike, Utc};
use anyhow::{anyhow, Error, Result};
use serde::{Serialize};


pub trait Configurable {
    /// Check that configuration is complete.
    /// This function shall be used at start up to detect misconfiguration as soon as possible
    /// It panics if configuration is incorrect.
    fn check_conf();
}

#[derive(Debug, Default, Serialize)]
pub struct Email {

    /// Who's sending this.
    #[serde(rename = "From")]
    pub from: String,

    /// Who to send to. Comma-delimited.
    #[serde(rename = "To")]
    pub to: String,

    /// Who to send to. Comma-delimited.
    #[serde(rename = "Subject")]
    pub subject: String,

    /// What to send (plaintext)
    #[serde(rename = "TextBody")]
    pub body: String,

    /// What to send (HTML)
    #[serde(rename = "HtmlBody")]
    pub bodyhtml: String,
}

impl Email {
    /// Construct a new `Email`.
    pub fn new(
        template_name: &str,
        to: &[String],
        subject: &str,
        mut context: Context,
        templates: Arc<RwLock<Tera>>,
    ) -> Result<Self, anyhow::Error> {
        let engine = templates
            .read()
            .map_err(|e| anyhow!("Error acquiring template read lock: {:?}", e))?;

        let now = Utc::now();
        let year = now.year();
        context.insert("year", &year.to_string());
        context.insert("subject", &subject);

        for (k,v) in env::vars() {
            if k.starts_with("JELLY_") {
                context.insert(k, &v);
            }
        }

        // some debug info
        info!("Context for template {} : {:?}", template_name, &context);

        let bodyhtml = engine.render(&(template_name.to_owned() + ".html"), &context).map_err(Error::msg)?;
        let body = engine.render(&(template_name.to_owned() + ".txt"), &context).map_err(Error::msg)?;

        Ok(Email {
            to: to.join(","),
            from: var("EMAIL_DEFAULT_FROM").expect("EMAIL_DEFAULT_FROM not set!"),
            bodyhtml: bodyhtml,
            body: body,
            subject : subject.to_string(),
        })
    }
}
