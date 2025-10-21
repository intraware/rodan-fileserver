use crate::{types, utils::auth::Claims};
use actix_web::{
    HttpMessage, HttpRequest, HttpResponse, Responder, get,
    http::header::{ContentDisposition, DispositionParam, DispositionType},
    web,
};
use jsonwebtoken::TokenData;
use std::path::Path;

#[get("/{challenge_id}/{file_name}")]
async fn get_file(req: HttpRequest, path: web::Path<(String, String)>) -> impl Responder {
    let (challenge_id, file_name) = path.into_inner();
    let ext = req.extensions();
    let token_data = ext.get::<TokenData<Claims>>().unwrap();
    let data_dir = req.app_data::<web::Data<types::FolderPath>>().unwrap();
    let file_path = format!(
        "{}/{}/{}/{}",
        data_dir.0, token_data.claims.team_id, challenge_id, file_name
    );
    match actix_files::NamedFile::open_async(file_path).await {
        Ok(file) => file
            .set_content_disposition(ContentDisposition {
                disposition: DispositionType::Attachment,
                parameters: vec![DispositionParam::Filename(file_name)],
            })
            .into_response(&req),
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

#[get("/{challenge_id}")]
async fn get_dir(req: HttpRequest, path: web::Path<String>) -> impl Responder {
    let challenge_id = path.into_inner();
    let ext = req.extensions();
    let token_data = ext.get::<TokenData<Claims>>().unwrap();
    let data_dir = req.app_data::<web::Data<types::FolderPath>>().unwrap();
    let dir_path = format!(
        "{}/{}/{}",
        data_dir.0, token_data.claims.team_id, challenge_id
    );
    let path = Path::new(dir_path.as_str());
    if !path.exists() || !path.is_dir() {
        return HttpResponse::NotFound().body("Directory not found");
    }
    match path.read_dir() {
        Ok(entries) => {
            let mut files = Vec::new();
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(file_name) = entry.file_name().to_str() {
                        files.push(file_name.to_string());
                    }
                }
            }
            HttpResponse::Ok()
                .content_type("application/json")
                .json(DirectoryResponse {
                    id: challenge_id,
                    files,
                })
        }
        Err(err) => {
            log::error!("Error reading directory: {}", err);
            HttpResponse::InternalServerError().body("Error reading directory")
        }
    }
}
