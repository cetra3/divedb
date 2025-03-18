use aes_gcm::aead::{generic_array::GenericArray, Aead};
use aes_gcm::Aes256Gcm; // Or `Aes128Gcm`
use anyhow::{anyhow, Error};
use base64::{decode, encode};
use rand::{thread_rng, Rng};
use uuid::Uuid;

use actix_web::{dev::Payload, FromRequest, HttpRequest};
use actix_web::{web, Error as ActixError};

use futures::future::{ready, Ready};

use crate::graphql::WebContext;

const NONCE_LEN: usize = 12;

pub trait TokenEncryptor {
    fn base64_encrypt<I: AsRef<[u8]>>(&self, input: I) -> Result<String, Error>;
    fn base64_decrypt<I: AsRef<[u8]>>(&self, input: I) -> Result<Vec<u8>, Error>;
}

impl TokenEncryptor for Aes256Gcm {
    fn base64_encrypt<I: AsRef<[u8]>>(&self, input: I) -> Result<String, Error> {
        let mut nonce = [0; NONCE_LEN];
        thread_rng().fill(&mut nonce);

        let mut ciphertext = self
            .encrypt(GenericArray::from_slice(&nonce), input.as_ref())
            .map_err(|err| anyhow!("{}", err))?;

        let mut output = Vec::new();

        output.extend_from_slice(&nonce);
        output.append(&mut ciphertext);

        Ok(encode(&output))
    }

    fn base64_decrypt<I: AsRef<[u8]>>(&self, input: I) -> Result<Vec<u8>, Error> {
        let bytes = decode(input.as_ref())?;

        if bytes.len() < NONCE_LEN {
            return Err(anyhow!("Invalid sized token"));
        }

        let (nonce, ciphertext) = bytes.split_at(NONCE_LEN);

        let output = self
            .decrypt(GenericArray::from_slice(nonce), ciphertext)
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
        if let Some(web) = req.app_data::<web::Data<WebContext>>() {
            if let Some(header) = req.headers().get("divedb-token") {
                return ready(Ok(Token {
                    user_id: web
                        .cipher
                        .base64_decrypt(header)
                        .ok()
                        .and_then(|val| TryInto::try_into(val).ok())
                        .map(Uuid::from_bytes),
                }));
            }
        }

        ready(Ok(Token { user_id: None }))
    }
}

#[cfg(test)]
mod tests {

    use crate::token::*;
    use aes_gcm::aead::{generic_array::GenericArray, NewAead};

    const KEY: &[u8; 32] = b"Really Secret Key!!!!!!!!!!!!!!!";
    const MESSAGE: &str = "Hello World";

    #[test]
    fn test_key() {
        let key = GenericArray::from_slice(KEY);
        let cipher = Aes256Gcm::new(key);

        let ciphertext = cipher.base64_encrypt(MESSAGE).expect("Encodes correctly");

        println!("Cipher:{}, Len:{}", ciphertext, ciphertext.len());

        let plaintext = String::from_utf8(
            cipher
                .base64_decrypt(ciphertext)
                .expect("Decrypts correctly"),
        )
        .expect("Decodes correctly");

        eprintln!("Plain:{}", plaintext);

        assert_eq!(&plaintext, MESSAGE)
    }
}
