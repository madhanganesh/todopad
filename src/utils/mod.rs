pub mod tags;

use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand_core::OsRng;

pub fn verify_password(hash: &str, plain: &str) -> bool {
    let parsed_hash = match PasswordHash::new(hash) {
        Ok(v) => v,
        Err(_) => {
            return false;
        }
    };

    let argon2 = Argon2::default();
    argon2
        .verify_password(plain.as_bytes(), &parsed_hash)
        .is_ok()
}

#[allow(dead_code)]
fn hash_password1(password: &str) {
    // Generate a random salt
    let salt = SaltString::generate(&mut OsRng);
    // Create the Argon2 instance with default parameters
    let argon2 = Argon2::default();
    // Hash the password
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    println!("Hashed password: {}", password_hash);

    // Verify the password
    //let parsed_hash = PasswordHash::new(&password_hash).unwrap();
    //let is_valid = argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok();
}
