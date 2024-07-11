use axum::{async_trait, extract::FromRequestParts, http::request::Parts};

use super::{claims::Claims, AuthError};


#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // todo: To be perfected
        // Extract the token from the authorization header
        // let TypedHeader(Authorization(bearer)) = parts
        //     .extract::<TypedHeader<Authorization<Bearer>>>()
        //     .await
        //     .map_err(|_| AuthError::InvalidToken)?;
        // // Decode the user data
        // let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
        //     .map_err(|_| AuthError::InvalidToken)?;

        Ok(Claims { uid: 0, exp: 1000 })
    }
}