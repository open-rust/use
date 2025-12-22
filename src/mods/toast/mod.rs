use axum::extract::Query;
use clap::Parser;
use serde::Deserialize;
use tower_http::cors::CorsLayer;

use crate::{ffi::{set_toast_alpha, set_toast_position, set_toast_wh, show_toast}, utils::ip::get_public_ip};

#[derive(Parser, Debug, Clone)]
#[command(version = "0.0.1", about = "Windows文本消息吐司API服务", long_about = "Windows文本消息吐司API服务
API:
- GET http://IP:PORT/toast?msg=123456&time=2000 (设置消息与显示时长)
- GET http://IP:PORT/setpos?x=1000&y=550 (设置窗口位置)
- GET http://IP:PORT/setwh?w=860&h=200 (设置窗口宽高)
- GET http://IP:PORT/setalpha?alpha=190 (设置窗口整体不透明度, 0为完全透明, 255为完全不透明)
")]
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

#[derive(Deserialize)]
struct Pos {
    x: i32,
    y: i32,
}

async fn handler_setpos(Query(pos): Query<Pos>) {
    set_toast_position(pos.x, pos.y);
}

#[derive(Deserialize)]
struct WH {
    w: i32,
    h: i32,
}

async fn handler_setwh(Query(wh): Query<WH>) {
    set_toast_wh(wh.w, wh.h);
}

#[derive(Deserialize)]
struct Alpha {
    alpha: u8,
}

async fn handler_set_alpha(Query(alpha): Query<Alpha>) {
    set_toast_alpha(alpha.alpha);
}

pub async fn main(param: Param) -> tokio::io::Result<()> {
    let router = axum::Router::new()
        .route("/toast", axum::routing::get(handler))
        .route("/setpos", axum::routing::get(handler_setpos))
        .route("/setwh", axum::routing::get(handler_setwh))
        .route("/setalpha", axum::routing::get(handler_set_alpha))
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
        set_toast_position(1000, 550);
    });
    axum::serve(listener, router).await?;
    Ok(())
}
