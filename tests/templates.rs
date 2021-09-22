#[macro_use]
extern crate lazy_static;
use dotenv;
use jelly::tera::Tera;
use std::env;

//    static ref ROOT: &'static Path = Path::new("data/");

lazy_static! {
    static ref TEMPLATES: Tera = {
        dotenv::dotenv().ok();
        let templates_glob = env::var("TEMPLATES_GLOB").expect("TEMPLATES_GLOB not set!");
        let templates = Tera::new(&templates_glob).expect("Unable to compile templates!");
        templates
    };
}

mod template_should_work_for {
    use super::*;
    /// Test that email templates render correctly with current .env.
    /// You should adapt the test to follow the settings in your .env and
    /// the template your use.

    #[allow(unused_imports)]
    use anyhow::{self, bail};
    use jelly::tera::Context;
    use log::debug;
    use mainlib::accounts::jobs;
    use std::env;
    use std::sync::{Arc, RwLock};
    use test_env_log::test;

    #[test]
    fn odd_registration_attempt() -> Result<(), anyhow::Error> {
        dotenv::dotenv().ok();
        let email = jelly::email::Email::new(
            "email/odd-registration-attempt",
            &["Erby Doe <test@example.com>".to_string()],
            "Test subject",
            jobs::build_odd_registration_attempt_context("John Doe"),
            Arc::new(RwLock::new(TEMPLATES.clone())),
        )?;

        assert_eq!(email.from, env::var("EMAIL_DEFAULT_FROM")?);
        assert_eq!(email.to, "Erby Doe <test@example.com>");
        assert_eq!(email.subject, "Test subject");
        debug!("{}", email.body);
        assert!(email.body.contains("accounts/reset"));
        Ok(())
    }

    #[test]
    fn reset_password() -> Result<(), anyhow::Error> {
        dotenv::dotenv().ok();
        let email = jelly::email::Email::new(
            "email/reset-password",
            &["Erby Doe <test@example.com>".to_string()],
            "Test subject",
            jobs::build_reset_password_context("/verify/xxxx"),
            Arc::new(RwLock::new(TEMPLATES.clone())),
        )?;

        assert_eq!(email.from, env::var("EMAIL_DEFAULT_FROM")?);
        assert_eq!(email.to, "Erby Doe <test@example.com>");
        assert_eq!(email.subject, "Test subject");
        debug!("{}", email.body);
        assert!(email.body.contains("/verify/xxxx"));
        //bail!("Not implemented!");
        Ok(())
    }

    #[test]
    fn verify_account() -> Result<(), anyhow::Error> {
        dotenv::dotenv().ok();
        let mut context = Context::new();
        context.insert("action_url", "/verify/account");
        let email = jelly::email::Email::new(
            "email/verify-account",
            &["Erby Doe <test@example.com>".to_string()],
            "Test subject",
            context,
            Arc::new(RwLock::new(TEMPLATES.clone())),
        )?;

        assert_eq!(email.from, env::var("EMAIL_DEFAULT_FROM")?);
        assert_eq!(email.to, "Erby Doe <test@example.com>");
        assert_eq!(email.subject, "Test subject");
        debug!("{}", email.body);
        assert!(email.body.contains("/verify/account"));
        //bail!("Not implemented!");
        Ok(())
    }

    #[test]
    fn welcome() -> Result<(), anyhow::Error> {
        dotenv::dotenv().ok();
        let email = jelly::email::Email::new(
            "email/welcome",
            &["Erby Doe <test@example.com>".to_string()],
            "Test subject",
            jobs::build_welcome_context("Erby Doe"),
            Arc::new(RwLock::new(TEMPLATES.clone())),
        )?;

        assert_eq!(email.from, env::var("EMAIL_DEFAULT_FROM")?);
        assert_eq!(email.to, "Erby Doe <test@example.com>");
        assert_eq!(email.subject, "Test subject");
        debug!("{}", email.body);
        assert!(email.body.contains("http://example.com/help"));
        //bail!("Not implemented!");
        Ok(())
    }
}
