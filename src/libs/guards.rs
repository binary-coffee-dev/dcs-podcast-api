use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};

use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};

use crate::libs::utils::get_env;

#[derive(Debug, Serialize, Deserialize)]
struct Jwt {
   id: String,
   role: String
}

fn is_valid_and_admin(key: &str) -> bool {
    let jwt_secret = get_env("JWT_SECRET", "THIS_IS_THE_REFERENCE");
    match String::from(key).get_mut(7..) {
        Some(jwt) => {
            match decode::<Jwt>(
                &jwt,
                &DecodingKey::from_secret(jwt_secret.as_ref()),
                &Validation::new(Algorithm::HS256)
            ) {
                Ok(jwt_obj) => {
                    jwt_obj.claims.role == String::from("administrator")
                },
                Err(_ignore) => false
            }
        },
        None => false
    }
}

pub struct AuthorizationGuard(String);

#[derive(Debug)]
pub enum ApiKeyError {
    BadCount,
    Missing,
    Invalid,
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthorizationGuard {
    type Error = ApiKeyError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        match keys.len() {
            0 => Outcome::Failure((Status::Unauthorized, ApiKeyError::Missing)),
            1 if is_valid_and_admin(keys[0]) => Outcome::Success(AuthorizationGuard(keys[0].to_string())),
            1 => Outcome::Failure((Status::Unauthorized, ApiKeyError::Invalid)),
            _ => Outcome::Failure((Status::BadRequest, ApiKeyError::BadCount)),
        }
    }
}
