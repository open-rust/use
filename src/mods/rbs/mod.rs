use core::server::Server;

use clap::Parser;

use crate::utils::ip::get_public_ip;

#[derive(Parser, Debug, Clone)]
#[command(version = "0.1.0", about = "内网穿透服务端")]
pub struct Param {
    #[arg(short, long, default_value_t = String::from("[::]"))]
    bind: String,

    #[arg(short, long, default_value_t = 1234)]
    port: u16,

    #[arg(long = "pwd", default_value_t = String::from("test"))]
    password: String,
}

pub async fn main(param: Param) -> tokio::io::Result<()> {
    let s = match Server::new(param.bind, param.port, param.password).await {
        Ok(v) => v,
        Err(e) => {
            macro_log::e!("Server start failed: {e}");
            return Err(e);
        }
    };
    macro_log::i!("Server started on {}:{}", get_public_ip(), param.port);
    s.serv().await;
    Ok(())
}
