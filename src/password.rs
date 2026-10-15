use anyhow::{Error, anyhow};
use scrypt::{
    Scrypt,
    password_hash::{PasswordHasher, PasswordVerifier, phc::PasswordHash},
};

/// Hashes a password as a PHC string (`$scrypt$ln=..,r=..,p=..$salt$hash`) using the recommended params
pub fn hash_password(password: &str) -> Result<String, Error> {
    let hash = Scrypt::default()
        .hash_password(password.as_bytes())
        .map_err(|err| anyhow!("{err}"))?;

    Ok(hash.to_string())
}

/// Checks a password against a PHC string, using the params stored within it
pub fn verify_password(password: &str, hash: &str) -> Result<(), Error> {
    let hash = PasswordHash::new(hash).map_err(|err| anyhow!("{err}"))?;

    Scrypt::default()
        .verify_password(password.as_bytes(), &hash)
        .map_err(|err| anyhow!("{err}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify() {
        let hash = hash_password("hunter2").expect("Hashes correctly");

        assert!(hash.starts_with("$scrypt$ln=17,r=8,p=1$"));
        assert!(verify_password("hunter2", &hash).is_ok());
        assert!(verify_password("hunter3", &hash).is_err());
    }
}
