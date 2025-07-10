use core::client::Client;
use core::client_p2p::ClientP2P;
use core::log::*;

use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(version = "0.1.0", about = "内网穿透客户端")]
pub struct Param {
    /// 服务器地址
    #[arg(short, long)]
    server: String,

    /// 本地服务地址
    #[arg(short, long, default_value_t = String::from("127.0.0.1:3389"))]
    local: String,

    /// 绑定的服务器端口
    #[arg(short, long, default_value_t = 13389)]
    port: u16,

    /// 绑定服务器端口所需密码
    #[arg(long = "pwd", default_value_t = String::from("test"))]
    password: String,

    /// 监听本地服务地址, 对服务器端口绑定的服务进行P2P访问
    #[arg(long, default_value_t = false)]
    p2p: bool,
}

pub async fn main(param: Param) -> tokio::io::Result<()> {
    loop {
        if param.p2p {
            serv_p2p(param.server.clone(), param.port, param.local.clone()).await;
        } else {
            serv(param.server.clone(), param.port, param.password.clone(), param.local.clone()).await;
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
    }
}

async fn serv_p2p(server: String, port: u16, local_service: String) {
    let p2p = ClientP2P::new(server.into(), port, local_service.into());
    match p2p.serv().await {
        Err(e) => {
            e!("启动失败：{e}");
        },
        _ => ()
    }
}

async fn serv(server: String, port: u16, password: String, local_service: String) {
    i!("正在连接服务器：{server}");
    let mut c = match Client::new(server.clone(), password).await {
        Ok(v) => v,
        Err(e) => {
            return e!("连接失败！{}", e.to_string());
        }
    };
    i!("正在绑定端口：{port}");
    match c.bind(port).await {
        Ok(()) => {
            let host = server.split(":").next().unwrap();
            i!("服务已绑定: {} -> {}:{}", local_service, host, port);
            c.proxy(local_service, |_task| {
                async move {
                    // task.abort();
                }
            }).await;
        }
        Err(e) => e!("绑定失败！{}", e.to_string()),
    };
}
