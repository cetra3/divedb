use anyhow::{Context, Error, anyhow, bail};
use async_trait::async_trait;
use base64::{Engine, engine::general_purpose::STANDARD};
use deadpool_postgres::Pool;
use divedb_core::FromRow;
use scrypt::{
    Params,
    password_hash::phc::{Output, PasswordHash, Salt},
    phc::ALG_ID,
};
use tracing::*;
use uuid::Uuid;

use super::Migration;

/// Converts password hashes from the `$rscrypt$` format of scrypt 0.3 to PHC strings, which is all
/// that newer versions of scrypt can verify
pub struct ScryptPhc;

#[async_trait]
impl Migration for ScryptPhc {
    fn name(&self) -> &str {
        "scrypt_phc"
    }

    async fn migrate(&self, pool: &Pool) -> Result<(), Error> {
        let mut client = pool.get().await?;
        let transaction = client.transaction().await?;

        let users = User::from_rows(
            transaction
                .query(
                    "select id, password from users where password like '$rscrypt$%'",
                    &[],
                )
                .await?,
        )?;

        let statement = transaction
            .prepare("update users set password = $1 where id = $2")
            .await?;

        for user in &users {
            let hash = rscrypt_to_phc(&user.password)
                .with_context(|| format!("Could not convert password hash for user {}", user.id))?;

            transaction.execute(&statement, &[&hash, &user.id]).await?;
        }

        transaction.commit().await?;

        info!("Converted {} password hashes to PHC strings", users.len());

        Ok(())
    }
}

/// `$rscrypt$<format>$<base64(log_n,r,p)>$<base64(salt)>$<base64(hash)>$` holds the same params, salt
/// and hash as a PHC string, so it can be re-encoded without the password
fn rscrypt_to_phc(rscrypt: &str) -> Result<String, Error> {
    let parts = rscrypt.split('$').collect::<Vec<_>>();

    let &["", "rscrypt", format, params, salt, hash, ""] = parts.as_slice() else {
        bail!("Unrecognised hash format");
    };

    // Format 0 stores `r` & `p` as single bytes, format 1 as little-endian u32s
    let (log_n, r, p) = match (format, STANDARD.decode(params)?.as_slice()) {
        ("0", &[log_n, r, p]) => (log_n, r.into(), p.into()),
        ("1", &[log_n, r0, r1, r2, r3, p0, p1, p2, p3]) => (
            log_n,
            u32::from_le_bytes([r0, r1, r2, r3]),
            u32::from_le_bytes([p0, p1, p2, p3]),
        ),
        _ => bail!("Unrecognised hash params"),
    };

    let params = Params::new(log_n, r, p).map_err(|err| anyhow!("{err}"))?;

    let hash = PasswordHash {
        algorithm: ALG_ID,
        version: None,
        params: params.try_into().map_err(|err| anyhow!("{err}"))?,
        salt: Some(Salt::new(&STANDARD.decode(salt)?).map_err(|err| anyhow!("{err}"))?),
        hash: Some(Output::new(&STANDARD.decode(hash)?).map_err(|err| anyhow!("{err}"))?),
    };

    Ok(hash.to_string())
}

#[derive(Debug, FromRow)]
struct User {
    id: Uuid,
    password: String,
}
