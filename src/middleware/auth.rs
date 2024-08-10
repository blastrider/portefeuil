use crate::handlers::Claims;
use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{decode, DecodingKey, Validation};

pub async fn jwt_middleware(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let token = credentials.token();
    let decoding_key = DecodingKey::from_secret("your_secret_key".as_ref());

    match decode::<Claims>(token, &decoding_key, &Validation::default()) {
        Ok(token_data) => {
            req.extensions_mut().insert(token_data.claims.sub);
            Ok(req)
        }
        Err(_err) => Err((actix_web::error::ErrorUnauthorized("Invalid token"), req)),
    }
}
