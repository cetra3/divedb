use anyhow::{anyhow, Error};
use reqwest::Url;
use serde::Deserialize;
use structopt::StructOpt;

use log::*;
use serde_json::Value;

#[derive(StructOpt, Clone, Debug, PartialEq)]
pub struct FacebookOauth {
    #[structopt(env, default_value = "")]
    pub fb_app_id: String,
    #[structopt(env, default_value = "")]
    fb_app_secret: String,
}

impl FacebookOauth {
    pub async fn email_from_code(
        &self,
        redirect_uri: String,
        code: String,
    ) -> Result<String, Error> {
        debug!("Redirect URI:{}", redirect_uri);

        let url = Url::parse_with_params(
            "https://graph.facebook.com/v8.0/oauth/access_token",
            &[
                ("client_id", &self.fb_app_id),
                ("redirect_uri", &redirect_uri),
                ("client_secret", &self.fb_app_secret),
                ("code", &code),
            ],
        )?;

        let response_value: Value = reqwest::get(url).await?.json().await?;

        if response_value.get("error").is_some() {
            let err_message = response_value
                .get("error")
                .and_then(|val| val.as_object())
                .and_then(|val| val.get("message"))
                .map(|val| val.to_string())
                .unwrap_or_else(|| String::from("No error specified"));

            return Err(anyhow!("Error from facebook: {}", err_message));
        }

        debug!("Response value from facebook:{}", response_value);

        let access_token: AccessToken = serde_json::from_value(response_value)?;

        let user_details_url = Url::parse_with_params(
            "https://graph.facebook.com/me",
            &[
                ("fields", "email"),
                ("access_token", &access_token.access_token),
            ],
        )?;

        let response_value: Value = reqwest::get(user_details_url).await?.json().await?;

        debug!(
            "User details response value from facebook:{}",
            response_value
        );
        if response_value.get("error").is_some() {
            let err_message = response_value
                .get("error")
                .and_then(|val| val.as_object())
                .and_then(|val| val.get("message"))
                .map(|val| val.to_string())
                .unwrap_or_else(|| String::from("No error specified"));

            return Err(anyhow!("Error from facebook: {}", err_message));
        }

        let user_details: UserDetails = serde_json::from_value(response_value)?;

        Ok(user_details.email)
    }
}

#[derive(Deserialize)]
pub struct AccessToken {
    access_token: String,
}

#[derive(Deserialize)]
pub struct UserDetails {
    email: String,
}
