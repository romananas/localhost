use crate::options;
use std::path::Path;

pub struct Response {
    pub file: String,
    pub status: u16,
}

pub fn get_file(path: &str, opts: &options::Opts) -> Response {
    let links = &opts.links;
    match links.get(path) {
        Some(v) => Response {
            file: v.clone(),
            status: 200,
        },
        None => Response {
            file: opts.not_found.clone(),
            status: 404,
        },
    }
}

pub fn get_status_line(code: u16) -> &'static str {
    match code {
        200 => "HTTP/1.1 200 OK",
        201 => "HTTP/1.1 201 Created",
        400 => "HTTP/1.1 400 Bad Request",
        404 => "HTTP/1.1 404 Not Found",
        500 => "HTTP/1.1 500 Internal Server Error",
        _ => "HTTP/1.1 500 Internal Server Error",
    }
}

/// Retourne le bon `Content-Type` en fonction de l'extension du fichier
pub fn get_content_type(file_path: &str) -> &str {
    let ext = Path::new(file_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");

    match ext {
        "html" => "text/html",
        "css" => "text/css",
        "js" => "application/javascript",
        "json" => "application/json",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "ico" => "image/x-icon",
        _ => "text/html",
    }
}