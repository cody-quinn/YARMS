use pbkdf2::{password_hash::{rand_core::OsRng, PasswordHasher, Result, PasswordHash, PasswordVerifier, SaltString}, Pbkdf2};

pub fn hash_password_rng_salt(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    hash_password(password, &salt)
}

pub fn hash_password(password: &str, salt: &SaltString) -> Result<String> {
    let password_byte_string = password.as_bytes();
    let password_hash = Pbkdf2.hash_password(password_byte_string, salt)?.to_string();
    Ok(password_hash)
}

pub fn check_password(password: &str, hash: &str) -> bool {
    match PasswordHash::new(hash) {
        Ok(hash) => Pbkdf2.verify_password(password.as_bytes(), &hash).is_ok(),
        Err(_) => return false,
    }
}
