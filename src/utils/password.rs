use argon2::{password_hash::{rand_core::OsRng, SaltString}, Argon2, PasswordHasher, PasswordVerifier};


pub type HashedPassword = String;
pub type UnhashedPassword = String;

pub fn hash_password(password: UnhashedPassword) -> HashedPassword {
    let salt=SaltString::generate(OsRng);
    let argon2=argon2::Argon2::default();
    let password_hash=argon2.hash_password(password.as_bytes(), &salt).unwrap();
    password_hash.to_string()
}

pub fn verify_password(hashed_password: &HashedPassword, unhashed_password: &UnhashedPassword) -> bool {
    let parsed_hash = argon2::PasswordHash::new(hashed_password).unwrap(); 
    Argon2::default().verify_password(unhashed_password.as_bytes(), &parsed_hash).is_ok()
}