use jsonwebtoken::{decode, Algorithm, DecodingKey, TokenData, Validation};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: u64,
    pub username: String,
    pub team_id: u64,
}

pub fn decode_jwt(token: &str, key: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_required_spec_claims(&["exp", "iss", "iat"]);
    validation.set_issuer(&["rodan"]);
    validation.validate_exp = true;
    decode::<Claims>(token, &DecodingKey::from_secret(key.as_bytes()), &validation)
}