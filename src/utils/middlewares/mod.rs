use crate::{
    types,
    utils::auth::{Claims, decode_jwt},
};
use actix_web::{
    Error, HttpMessage as _,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error::ErrorUnauthorized,
    middleware::Next,
    web,
};
use jsonwebtoken::TokenData;

pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let auth = req.headers().get("Authorization");
    if auth.is_none() {
        return Err(ErrorUnauthorized("Authorization header is missing"));
    }
    let auth = auth.unwrap().to_str().unwrap_or("");
    if !auth.starts_with("Bearer ") {
        return Err(ErrorUnauthorized(
            "Authorization header must start with 'Bearer '",
        ));
    }
    let token = &auth[7..];
    let key = req.app_data::<web::Data<types::JwtSecret>>().unwrap();
    let token_data: TokenData<Claims> = match decode_jwt(token, &key.0) {
        Ok(data) => data,
        Err(err) => {
            log::error!("Error decoding token: {}", err);
            return Err(ErrorUnauthorized("Invalid token"));
        }
    };
    req.extensions_mut().insert(token_data);
    next.call(req).await
}
