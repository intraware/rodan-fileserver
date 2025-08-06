
use actix_web::{body::MessageBody, dev::{ServiceRequest, ServiceResponse}, error::ErrorUnauthorized, middleware::Next, web, Error, HttpMessage as _};
use jsonwebtoken::TokenData;
use crate::auth::{decode_jwt, Claims};

pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let auth = req.headers().get("Authorization");
    if auth.is_none() {
        return  Err(ErrorUnauthorized("Authorization header is missing"));
    }
    let auth = auth.unwrap().to_str().unwrap_or("");
    if !auth.starts_with("Bearer ") {
        return Err(ErrorUnauthorized("Authorization header must start with 'Bearer '"));
    }
    let token = &auth[7..];
    let key = req.app_data::<web::Data<String>>().unwrap();
    let token_data: TokenData<Claims> = match decode_jwt(token,&key) {
        Ok(data) => data,
        Err(err) => {
            log::error!("Error decoding token: {}", err);
            return Err(ErrorUnauthorized("Invalid token"));
        }
    };
    req.extensions_mut().insert(token_data);
    next.call(req).await
}