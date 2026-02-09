use sha_crypt::{sha512_simple, Sha512Params, ROUNDS_DEFAULT};

pub(crate) fn hash_password_sha512(password: &str) -> anyhow::Result<String> {
    let params =
        Sha512Params::new(ROUNDS_DEFAULT).map_err(|e| anyhow::anyhow!("invalid sha512-crypt params: {e:?}"))?;
    sha512_simple(password, &params).map_err(|e| anyhow::anyhow!("password hashing failed: {e:?}"))
}
