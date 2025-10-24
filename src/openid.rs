use std::collections::HashMap;

use anyhow::{anyhow, Error};
use clap::Parser;

use openidconnect::{
    AccessTokenHash, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EndpointMaybeSet,
    EndpointNotSet, EndpointSet, IssuerUrl, Nonce, OAuth2TokenResponse, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, Scope, TokenResponse,
};

use openidconnect::core::{
    CoreAuthenticationFlow, CoreClient, CoreProviderMetadata, CoreUserInfoClaims,
};
use tokio::sync::RwLock;
use tracing::info;
use url::Url;

use crate::SITE_URL;

#[derive(Parser, Clone, Debug, PartialEq)]
pub struct OpenIDSettings {
    #[arg(long, env = "OPENID_ISSUER_NAME")]
    pub issuer_name: String,
    #[arg(long, env = "OPENID_ISSUER_URL")]
    pub issuer_url: String,
    #[arg(long, env = "OPENID_CLIENT_ID")]
    pub client_id: String,
    #[arg(long, env = "OPENID_CLIENT_SECRET")]
    pub client_secret: String,
}

type AuthClient = CoreClient<
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointMaybeSet,
    EndpointMaybeSet,
>;

pub struct OpenIDClient {
    issuer_name: String,
    http_client: reqwest::Client,
    client: AuthClient,
    request_map: RwLock<HashMap<String, (PkceCodeVerifier, Nonce)>>,
}

pub struct OpenIDResponse {
    pub email: String,
    pub preferred_username: String,
}

impl OpenIDClient {
    pub async fn new(settings: &OpenIDSettings, client: reqwest::Client) -> Result<Self, Error> {
        info!(
            "Discovering OpenID Connect provider metadata at url: {}",
            settings.issuer_url
        );
        let provider_metadata = CoreProviderMetadata::discover_async(
            IssuerUrl::new(settings.issuer_url.clone())?,
            &client,
        )
        .await?;

        info!(
            "Provider metadata discovered: {:?}",
            provider_metadata.authorization_endpoint()
        );

        let auth_client = CoreClient::from_provider_metadata(
            provider_metadata,
            ClientId::new(settings.client_id.clone()),
            Some(ClientSecret::new(settings.client_secret.clone())),
        )
        // Set the URL the user will be redirected to after the authorization process.
        .set_redirect_uri(RedirectUrl::new(format!("{}/oauth/callback", &*SITE_URL))?);

        Ok(Self {
            issuer_name: settings.issuer_name.clone(),
            http_client: client,
            client: auth_client,
            request_map: RwLock::new(HashMap::new()),
        })
    }

    pub fn issuer_name(&self) -> String {
        self.issuer_name.clone()
    }

    pub async fn request_authorization_url(&self) -> Result<Url, Error> {
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        let (auth_url, csrf_token, nonce) = self
            .client
            .authorize_url(
                CoreAuthenticationFlow::AuthorizationCode,
                CsrfToken::new_random,
                Nonce::new_random,
            )
            .set_pkce_challenge(pkce_challenge)
            .add_scope(Scope::new("profile".to_string()))
            .add_scope(Scope::new("email".to_string()))
            .url();

        self.request_map
            .write()
            .await
            .insert(csrf_token.into_secret(), (pkce_verifier, nonce));

        Ok(auth_url)
    }

    pub async fn validate_callback(
        &self,
        code: &str,
        state: &str,
    ) -> Result<OpenIDResponse, Error> {
        let (pkce_verifier, nonce) = self
            .request_map
            .write()
            .await
            .remove(state)
            .ok_or_else(|| anyhow!("No Active Authentication Request Found"))?;

        let token_response = self
            .client
            .exchange_code(AuthorizationCode::new(code.to_string()))?
            .set_pkce_verifier(pkce_verifier)
            .request_async(&self.http_client)
            .await?;

        let id_token = token_response
            .id_token()
            .ok_or_else(|| anyhow!("Missing ID Token"))?;
        let id_token_verifier = self.client.id_token_verifier();
        let claims = id_token.claims(&id_token_verifier, &nonce)?;

        if let Some(expected_access_token_hash) = claims.access_token_hash() {
            let actual_access_token_hash = AccessTokenHash::from_token(
                token_response.access_token(),
                id_token.signing_alg()?,
                id_token.signing_key(&id_token_verifier)?,
            )?;
            if actual_access_token_hash != *expected_access_token_hash {
                return Err(anyhow!("Invalid access token"));
            }
        }

        let userinfo: CoreUserInfoClaims = self
            .client
            .user_info(token_response.access_token().to_owned(), None)?
            .request_async(&self.http_client)
            .await?;

        let email = claims
            .email()
            .or(userinfo.email())
            .ok_or_else(|| anyhow!("Missing email in claim/user info"))?
            .to_string();
        let preferred_username = claims
            .preferred_username()
            .or(userinfo.preferred_username())
            .ok_or_else(|| anyhow!("Missing preferred username in claim/user info"))?
            .to_string();

        Ok(OpenIDResponse {
            email,
            preferred_username,
        })
    }
}
