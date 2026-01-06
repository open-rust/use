use axum::{
    extract::State,
    http::{HeaderValue, Request, StatusCode, header},
    middleware::Next,
    response::Response,
};
use tokio::{fs::File, io::AsyncReadExt};

use crate::{mods::fs::Param, utils::fs::normalize_path};

// 中间件：在 ServeDir 返回的响应中检查 gzip 文件并添加 Header
pub async fn add_gzip_encoding(
    State(param): State<Param>,
    request: Request<axum::body::Body>,
    next: Next,
) -> Response {
    let uri = request.uri().path().to_string();
    assert!(uri.starts_with('/'));

    let mut response = next.run(request).await;
    if response.status() == StatusCode::OK {
        let path = format!("{}{}", param.dir, normalize_path(&uri));
        macro_log::d!("path: {}", path);

        if is_gzip_file(&path).await {
            response
                .headers_mut()
                .insert(header::CONTENT_ENCODING, HeaderValue::from_static("gzip"));
        }
    }

    response
}

// 判断是否是 gzip 文件（魔数法）
async fn is_gzip_file(path: &str) -> bool {
    let mut file = match File::open(path).await {
        Ok(f) => f,
        Err(_) => return false,
    };

    let mut buf = [0u8; 2];
    if file.read_exact(&mut buf).await.is_err() {
        return false;
    }
    macro_log::d!("buf: {:?}", buf);

    // 不必 seek 回开头
    // let _ = file.seek(tokio::io::SeekFrom::Start(0)).await;

    buf == [0x1f, 0x8b]
}
