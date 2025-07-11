mod ls;

use axum::response::Html;
use clap::Parser;

use crate::utils::ip::{get_interface_ipv4s, get_public_ip};

#[derive(Parser, Debug, Clone)]
#[command(version = "0.0.1", about = "简易文件浏览器")]
pub struct Param {
    /// 绑定地址
    #[arg(long, short, default_value_t = String::from("0.0.0.0"))]
    pub bind: String,

    /// 绑定端口
    #[arg(long, short, default_value_t = 0)]
    pub port: u16,

    /// 指定目录
    #[arg(long, short, default_value_t = String::from("."))]
    pub dir: String,
}

pub async fn main(param: Param) -> tokio::io::Result<()> {
    let router = axum::Router::new()
        .route(
            "/",
            axum::routing::get(|| async { Html(include_bytes!("index.html")) }),
        )
        .route("/ls", axum::routing::get(ls::ls))
        .fallback_service(tower_http::services::ServeDir::new(&param.dir))
        .with_state(param.clone())
        .into_make_service();
    let listener =
        tokio::net::TcpListener::bind(format!("{}:{:?}", param.bind, param.port)).await?;
    let port = listener.local_addr().unwrap().port();
    tokio::task::spawn_blocking(move || {
        let ip = get_public_ip();
        macro_log::i!("Public: http://{}:{}", ip, port);
    });
    get_interface_ipv4s().iter().for_each(|ip| {
        macro_log::i!("Local: http://{}:{}", ip, port);
    });
    axum::serve(listener, router).await?;
    Ok(())
}
