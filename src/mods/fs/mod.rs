mod ls;
mod serve_dir;

use axum::{middleware, response::Html};
use clap::Parser;
use tower_http::cors::CorsLayer;

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

    /// 指定预压缩的 gzip 文件后缀名，如：.gz .terrain
    #[arg(long, short, default_values_t = vec![String::from(".gz"), String::from(".terrain")])]
    pub gzip: Vec<String>,

    /// 是否即使命中文件后缀名，也要读取文件判断是否为 gzip 文件
    #[arg(long = "gf")]
    pub gzip_file: bool,
}

pub async fn main(param: Param) -> tokio::io::Result<()> {
    // 仅对 fallback 应用中间件
    let router = axum::Router::new()
        // .fallback_service(tower_http::services::ServeDir::new(&param.dir)) // 必须有 route 才能 fallback
        .route_service("/{*wildcard}", tower_http::services::ServeDir::new(&param.dir))
        .route_layer(middleware::from_fn_with_state(param.clone(), serve_dir::add_gzip_encoding));
    // 合并路由或者设置 fallback 路由
    let router = axum::Router::new()
        .route(
            "/",
            axum::routing::get(|| async { Html(include_bytes!("index.html")) }),
        )
        .route("/ls", axum::routing::get(ls::ls))
        // .merge(router) // 合并路由
        .fallback_service(router) // 设置 fallback 路由
        .layer(CorsLayer::permissive())
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
