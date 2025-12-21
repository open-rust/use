use axum::extract::Query;
use clap::Parser;
use serde::Deserialize;
use tower_http::cors::CorsLayer;

use crate::{ffi::show_toast, utils::ip::get_public_ip};

#[derive(Parser, Debug, Clone)]
#[command(version = "0.0.1", about = "Windows文本消息吐司API服务
API:
- GET http://IP:PORT/toast?msg=123456&time=2000")]
pub struct Param {
    /// 绑定地址
    #[arg(long, short, default_value_t = String::from("127.0.0.1"))]
    pub bind: String,

    /// 绑定端口
    #[arg(long, short, default_value_t = 7878)]
    pub port: u16,
}

#[derive(Deserialize)]
struct Toast {
    msg: String,
    time: Option<u32>,
}

async fn handler(Query(toast): Query<Toast>) {
    show_toast(&toast.msg, toast.time.unwrap_or_default());
}

pub async fn main(param: Param) -> tokio::io::Result<()> {
    let router = axum::Router::new()
        .route("/toast", axum::routing::get(handler))
        .layer(CorsLayer::permissive())
        .with_state(param.clone())
        .into_make_service();
    let listener =
        tokio::net::TcpListener::bind(format!("{}:{:?}", param.bind, param.port)).await?;
    let port = listener.local_addr().unwrap().port();
    tokio::task::spawn_blocking(move || {
        if param.bind == "0.0.0.0" {
            let ip = get_public_ip();
            macro_log::i!("Public API: GET http://{}:{}/toast?msg=123456&time=2000", ip, port);
        } else {
            macro_log::i!("API: GET http://{}:{}/toast?msg=123456&time=2000", param.bind, port);
        }
        show_toast("服务已启动, 调用/toast接口, 您的消息将在这里显示", 0);
    });
    axum::serve(listener, router).await?;
    Ok(())
}
