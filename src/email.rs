use crate::{
    schema::{EmailVerification, PasswordReset},
    ConfigContext, EmailSecurity, SITE_URL,
};
use anyhow::{anyhow, Result};
use askama::Template;
use lettre::{
    message::{header, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use uuid::Uuid;
pub struct Emailer {
    mailer: AsyncSmtpTransport<Tokio1Executor>,
    from_addr: String,
    site_url: String,
}

impl Emailer {
    pub fn new(context: &ConfigContext) -> Self {
        let mut transort_builder = match context.email_settings.smtp_security {
            EmailSecurity::None => AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(
                &context.email_settings.smtp_host,
            ),
            EmailSecurity::Tls => {
                AsyncSmtpTransport::<Tokio1Executor>::relay(&context.email_settings.smtp_host)
                    .expect("Could not build smtp tls transport")
            }
        };

        if let (Some(ref user), Some(ref pass)) = (
            &context.email_settings.smtp_username,
            &context.email_settings.smtp_password,
        ) {
            transort_builder =
                transort_builder.credentials(Credentials::new(user.clone(), pass.clone()));
        }

        let mailer = transort_builder
            .port(context.email_settings.smtp_port)
            .build();
        let from_addr = context.email_settings.from_addr.clone();

        Self {
            mailer,
            from_addr,
            site_url: SITE_URL.clone(),
        }
    }

    pub async fn password_reset(&self, email: String, reset: PasswordReset) -> Result<()> {
        let mjml = HtmlReset {
            id: reset.id,
            email: &email,
            site_url: &self.site_url,
        }
        .render()?;
        let text = TextReset {
            id: reset.id,
            email: &email,
            site_url: &self.site_url,
        }
        .render()?;

        let root = mrml::parse(&mjml).map_err(|val| anyhow!("{:?}", val))?;
        let opts = mrml::prelude::render::Options::default();

        let html = root.render(&opts).map_err(|val| anyhow!("{:?}", val))?;

        let email = Message::builder()
            .from(self.from_addr.parse()?)
            .to(email.parse()?)
            .subject("DiveDB Password Reset")
            .multipart(
                MultiPart::alternative() // This is composed of two parts.
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_PLAIN)
                            .body(text), // Every message should have a plain text fallback.
                    )
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_HTML)
                            .body(html),
                    ),
            )?;

        self.mailer.send(email).await?;

        Ok(())
    }

    pub async fn email_verification(
        &self,
        email: String,
        verification: EmailVerification,
    ) -> Result<()> {
        let mjml = HtmlVerification {
            id: verification.id,
            email: &email,
            site_url: &self.site_url,
        }
        .render()?;
        let text = TextVerification {
            id: verification.id,
            email: &email,
            site_url: &self.site_url,
        }
        .render()?;

        let root = mrml::parse(&mjml).map_err(|val| anyhow!("{:?}", val))?;
        let opts = mrml::prelude::render::Options::default();

        let html = root.render(&opts).map_err(|val| anyhow!("{:?}", val))?;

        let email = Message::builder()
            .from(self.from_addr.parse()?)
            .to(email.parse()?)
            .subject("DiveDB Email Verification")
            .multipart(
                MultiPart::alternative() // This is composed of two parts.
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_PLAIN)
                            .body(text), // Every message should have a plain text fallback.
                    )
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_HTML)
                            .body(html),
                    ),
            )?;

        self.mailer.send(email).await?;

        Ok(())
    }
}

#[derive(Template)]
#[template(path = "password_reset.mjml", escape = "none")]
struct HtmlReset<'a> {
    id: Uuid,
    email: &'a str,
    site_url: &'a str,
}

#[derive(Template)]
#[template(path = "password_reset.txt", escape = "none")]
struct TextReset<'a> {
    id: Uuid,
    email: &'a str,
    site_url: &'a str,
}

#[derive(Template)]
#[template(path = "email_verification.mjml", escape = "none")]
struct HtmlVerification<'a> {
    id: Uuid,
    email: &'a str,
    site_url: &'a str,
}

#[derive(Template)]
#[template(path = "email_verification.txt", escape = "none")]
struct TextVerification<'a> {
    id: Uuid,
    email: &'a str,
    site_url: &'a str,
}
