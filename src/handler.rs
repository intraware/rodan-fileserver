use std::path::Path;
use jsonwebtoken::TokenData;
use actix_web::{get, http::header::{ContentDisposition, DispositionParam, DispositionType}, web, HttpRequest, HttpResponse, Responder};
use crate::auth::{decode_jwt, Claims};

const DATA_DIR: &str = "data";

#[get("/{container_id}/{file_name}")]
async fn get_file(req: HttpRequest, path: web::Path<(String, String)>, key: web::Data<String>) -> impl Responder {
    let auth = req.headers().get("Authorization");
    if auth.is_none() {
        return HttpResponse::Unauthorized().body("Authorization header is missing");
    }
    let auth = auth.unwrap().to_str().unwrap_or("");
    if !auth.starts_with("Bearer ") {
        return HttpResponse::Unauthorized().body("Authorization header must start with 'Bearer '");
    }
    let token = &auth[7..];
    let token_data: TokenData<Claims> = match decode_jwt(token, key.get_ref()) {
        Ok(data) => data,
        Err(err) => {
            log::error!("Error decoding token: {}", err);
            return HttpResponse::Unauthorized().body("Invalid token");
        }
    };
    
    let (container_id, file_name) = path.into_inner();
    let file_path = format!("{}/{}/{}/{}", DATA_DIR, token_data.claims.team_id, container_id, file_name);
    
    match actix_files::NamedFile::open_async(file_path).await {
        Ok(file) => file.set_content_disposition(ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![DispositionParam::Filename(file_name)],
        }).into_response(&req),
        Err(err) => {
            log::error!("Error opening file: {}", err);
            HttpResponse::NotFound().body("File not found")
        }
    }
}


#[derive(Debug, serde::Serialize)]
struct DirectoryResponse {
    id: String,
    files: Vec<String>,
}

#[get("/{container_id}")]
async fn get_dir(req: HttpRequest, path: web::Path<String>, key: web::Data<String>) -> impl Responder {
    let auth = req.headers().get("Authorization");
    if auth.is_none() {
        return HttpResponse::Unauthorized().body("Authorization header is missing");
    }
    let auth = auth.unwrap().to_str().unwrap_or("");
    if !auth.starts_with("Bearer ") {
        return HttpResponse::Unauthorized().body("Authorization header must start with 'Bearer '");
    }
    let token = &auth[7..];
    let token_data: TokenData<Claims> = match decode_jwt(token, key.get_ref()) {
        Ok(data) => data,
        Err(err) => {
            log::error!("Error decoding token: {}", err);
            return HttpResponse::Unauthorized().body("Invalid token");
        }
    };
    let container_id= path.into_inner();
    let dir_path = format!("{}/{}/{}", DATA_DIR, token_data.claims.team_id, container_id);
    let path = Path::new(dir_path.as_str());
    if !path.exists() || !path.is_dir() {
        return HttpResponse::NotFound().body("Directory not found");
    }
    match path.read_dir() {
        Ok(entries) => {
            let mut files = Vec::new();
            for entry in entries {
                if let Ok(entry) = entry {
                    // let file_name = ;
                    if let Some(file_name) = entry.file_name().to_str() {
                        files.push(file_name.to_string());
                    }
                }
            }
            HttpResponse::Ok().content_type("application/json").json(DirectoryResponse {
                id: container_id,
                files,
            })
        }
        Err(err) => {
            log::error!("Error reading directory: {}", err);
            HttpResponse::InternalServerError().body("Error reading directory")
        }
    }
}