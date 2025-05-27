use bcrypt::verify;

pub struct Hash {}

impl Hash {
    pub fn verify_password(passwordhash: &str, hash: &str) -> bool {
        verify(passwordhash, hash).unwrap_or(false)
    }
}
