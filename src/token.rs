use aes_gcm::Aes256Gcm; // Or `Aes128Gcm`
use aes_gcm::aead::{Aead, Nonce};
use anyhow::{Error, anyhow};
use base64::{Engine, engine::general_purpose::STANDARD};
use uuid::Uuid;

use actix_web::{Error as ActixError, web};
use actix_web::{FromRequest, HttpRequest, dev::Payload};

use futures::future::{Ready, ready};

use crate::graphql::WebContext;

const NONCE_LEN: usize = 12;

pub trait TokenEncryptor {
    fn base64_encrypt<I: AsRef<[u8]>>(&self, input: I) -> Result<String, Error>;
    fn base64_decrypt<I: AsRef<[u8]>>(&self, input: I) -> Result<Vec<u8>, Error>;
}

impl TokenEncryptor for Aes256Gcm {
    fn base64_encrypt<I: AsRef<[u8]>>(&self, input: I) -> Result<String, Error> {
        let mut nonce = [0; NONCE_LEN];
        rand::fill(&mut nonce);

        let mut ciphertext = self
            .encrypt(&Nonce::<Self>::from(nonce), input.as_ref())
            .map_err(|err| anyhow!("{}", err))?;

        let mut output = Vec::new();

        output.extend_from_slice(&nonce);
        output.append(&mut ciphertext);

        Ok(STANDARD.encode(&output))
    }

    fn base64_decrypt<I: AsRef<[u8]>>(&self, input: I) -> Result<Vec<u8>, Error> {
        let bytes = STANDARD.decode(input.as_ref())?;

        if bytes.len() < NONCE_LEN {
            return Err(anyhow!("Invalid sized token"));
        }

        let (nonce, ciphertext) = bytes.split_at(NONCE_LEN);
        let nonce = Nonce::<Self>::try_from(nonce)?;

        let output = self
            .decrypt(&nonce, ciphertext)
            .map_err(|err| anyhow!("{}", err))?;

        Ok(output)
    }
}

pub struct Token {
    pub user_id: Option<Uuid>,
}

impl FromRequest for Token {
    type Error = ActixError;
    type Future = Ready<Result<Token, ActixError>>;

    fn from_request(req: &HttpRequest, _pl: &mut Payload) -> Self::Future {
        if let Some(web) = req.app_data::<web::Data<WebContext>>()
            && let Some(header) = req.headers().get("divedb-token")
        {
            return ready(Ok(Token {
                user_id: web
                    .cipher
                    .base64_decrypt(header)
                    .ok()
                    .and_then(|val| TryInto::try_into(val).ok())
                    .map(Uuid::from_bytes),
            }));
        }

        ready(Ok(Token { user_id: None }))
    }
}

#[cfg(test)]
mod tests {

    use crate::token::*;
    use aes_gcm::KeyInit;

    const KEY: &[u8; 32] = b"Really Secret Key!!!!!!!!!!!!!!!";
    const MESSAGE: &str = "Hello World";

    #[test]
    fn test_key() {
        let cipher = Aes256Gcm::new(KEY.into());

        let ciphertext = cipher.base64_encrypt(MESSAGE).expect("Encodes correctly");

        println!("Cipher:{}, Len:{}", ciphertext, ciphertext.len());

        let plaintext = String::from_utf8(
            cipher
                .base64_decrypt(ciphertext)
                .expect("Decrypts correctly"),
        )
        .expect("Decodes correctly");

        eprintln!("Plain:{plaintext}");

        assert_eq!(&plaintext, MESSAGE)
    }
}
