use axum::{extract::{RawQuery, State}, http::StatusCode, response::IntoResponse, Json};

use crate::{mods::fs::Param, utils::fs::normalize_path};

#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
pub struct Response {
    pwd: String,
    dirs: Vec<String>,
    files: Vec<String>,
}

pub async fn ls(RawQuery(dir): RawQuery, State(param): State<Param>) -> Result<impl IntoResponse, impl IntoResponse> {
    let Some(raw_dir) = dir else {
        return Err((StatusCode::BAD_REQUEST, "BAD_REQUEST".into()));
    };
    // 相对路径
    let dir = format!("{}/{}", param.dir, normalize_path(&raw_dir));
    macro_log::i!("ls: {}", normalize_path(&dir));
    tokio::task::spawn(async move {
        let Ok(mut dirs) = tokio::fs::read_dir(&dir).await else {
            return Err((StatusCode::BAD_REQUEST, format!("Error read dir: {:?}", normalize_path(&dir))));
        };
        let mut resp = Response::default();
        resp.pwd = normalize_path(&raw_dir);
        loop {
            if let Ok(Some(dir)) = dirs.next_entry().await {
                let name = dir.path().file_name().unwrap_or_default().to_string_lossy().to_string();
                match dir.file_type().await {
                    Ok(file_type) if file_type.is_dir() => {
                        resp.dirs.push(normalize_path(&name));
                    }
                    Ok(file_type) if file_type.is_file() => {
                        resp.files.push(normalize_path(&name));
                    }
                    Ok(file_type) if file_type.is_symlink() => {
                        if let Ok(it) = tokio::fs::metadata(dir.path()).await {
                            match true {
                                true if it.is_dir() => resp.dirs.push(normalize_path(&name)),
                                true if it.is_file() => resp.files.push(normalize_path(&name)),
                                _ => ()
                            }
                        }
                    }
                    Ok(_) => {
                        macro_log::e!("Unsupported file: {}", name);
                    }
                    _ => {
                        macro_log::e!("Error getting file type: {}", name);
                    }
                }
            } else {
                return Ok(Json(resp));
            }
        }
    })
    .await
    .unwrap_or(Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        "INTERNAL_SERVER_ERROR".into(),
    )))
}
