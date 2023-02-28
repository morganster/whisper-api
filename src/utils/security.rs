use crate::{
    schema::models::users::{self, Claims},
    ApiError,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use scrypt::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Scrypt,
};

#[derive(Clone)]
pub struct Token(pub String);

pub fn get_hashed_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Scrypt
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();
    password_hash
}

pub fn verify_password(password: &str, password_hash: &str) -> bool {
    let parsed_hash = PasswordHash::new(password_hash).unwrap();
    Scrypt
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

pub fn get_secret() -> Vec<u8> {
    std::env::var("JWT_SECRET").unwrap().into_bytes()
}

pub fn get_token_data(token: &Token) -> Result<TokenData<Claims>, ApiError> {
    let token_data = decode::<Claims>(
        &token.0.to_string(),
        &DecodingKey::from_secret(&get_secret()),
        &Validation::default(),
    );
    match token_data {
        Ok(data) => Ok(data),
        Err(_e) => Err(ApiError::InvalidToken),
    }
}

pub fn get_jwt_for_user(user: &users::User) -> String {
    let expiration_time = Utc::now()
        .checked_add_signed(Duration::days(60))
        .expect("invalid timestamp")
        .timestamp();
    let user_claims = users::Claims {
        sub: user.id.clone(),
        exp: expiration_time as usize,
    };
    let token = match encode(
        &Header::default(),
        &user_claims,
        &EncodingKey::from_secret(&get_secret()),
    ) {
        Ok(t) => t,
        Err(_) => panic!(),
    };
    token
}
