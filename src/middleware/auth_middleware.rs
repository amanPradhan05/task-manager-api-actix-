use actix_web::{dev, Error, FromRequest, HttpRequest};
use futures_util::future::{ready, Ready};
use jsonwebtoken::{decode, Algorithm, DecodingKey, TokenData, Validation};
use std::pin::Pin;

pub struct AuthenticatedUser {
    pub user_id: i32,
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<AuthenticatedUser, Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        let auth_header = req.headers().get("Authorization");

        if let Some(auth_header) = auth_header {
            if let Ok(auth_token) = auth_header.to_str() {
                if let Some(token) = auth_token.strip_prefix("Bearer ") {
                    if let Ok(token_data) = decode_token(token) {
                        return ready(Ok(AuthenticatedUser {
                            user_id: token_data.claims.user_id,
                        }));
                    }
                }
            }
        }

        ready(Err(Error::from(())))
    }
}

fn decode_token(token: &str) -> Result<TokenData<TokenClaims>, jsonwebtoken::errors::Error> {
    let decoding_key = DecodingKey::from_secret(b"your-jwt-secret");
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<TokenClaims>(token, &decoding_key, &validation)?;
    Ok(token_data)
}

#[derive(Debug, Deserialize)]
struct TokenClaims {
    user_id: i32,
}
