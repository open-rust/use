mod config;
mod api;

use core::{*, time::get_time};
use api::*;
use clap::Parser;
use config::*;
use std::{net::SocketAddr, time::Duration, io};

use tokio::{net::{TcpListener, TcpStream}, io::AsyncWriteExt, time::sleep, fs};

use crate::utils;

#[derive(Parser, Debug, Clone)]
#[command(version = "0.0.1", about = "端口复用")]
pub struct Param {
    /// 配置文件路径
    #[arg(short, long, default_value_t = String::from("config.yml"))]
    config: String,

    /// 弹出配置文件模板
    #[arg(short, long, default_value_t = false)]
    eject: bool,

    /// 热重启
    #[arg(short, long, default_value_t = false)]
    reload: bool,
}

pub async fn main(param: Param) -> tokio::io::Result<()> {
    panic::custom_panic();
    let config_file = Some(param.config);
    if param.eject {
        return eject(config_file.clone()).await;
    }
    if param.reload {
        reload(config_file.clone()).await;
        return Ok(());
    }
    loop {
        boot(config_file.clone()).await;
        sleep(Duration::from_millis(5000)).await;
    }
}

/// 弹出配置文件模板
async fn eject(config_file: Option<String>) -> tokio::io::Result<()> {
    let config = match config_file {
        Some(v) => v,
        None => return Ok(()),
    };
    let mut config = utils::fs::normalize_path(&config);
    if !config.starts_with("/") {
        config = format!("./{config}");
    }
    let dir_config = config.trim_end_matches(|c| c != '/').trim_end_matches('/');
    if !std::fs::exists(dir_config)? {
        macro_log::i!("Creating dir: {}", dir_config);
        std::fs::create_dir_all(dir_config)?;
    }
    macro_log::i!("Creating file: {}", config);
    tokio::fs::write(config, include_bytes!("config.yml")).await?;
    Ok(())
}

/// 热重启, 目前暂时不支持修改热重启接口, 修改将导致无法再次通过命令行进行热重启
async fn reload(config_file: Option<String>) {
    let config = match read_config(config_file).await {
        Some(v) => v,
        None => return,
    };
    match load_config(&config).await {
        Ok((_listen, api)) => {
            match TcpStream::connect(api).await {
                Ok(mut stream) => {
                    let _ = stream.write_all(b"GET /oneport/reload HTTP/1.1\r\nHost: localhost\r\n\r\n").await;
                    i!("Restarting...");
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                }
                _ => ()
            }
        },
        Err(e) => {
            return e!("Config load failed: {e}");
        }
    }
}

async fn boot(config_file: Option<String>) {
    let config = match read_config(config_file).await {
        Some(v) => v,
        None => return,
    };
    let (listen, api) = match load_config(&config).await {
        Ok((listen, api)) => {
            i!("Config loaded");
            (listen, api)
        },
        Err(e) => {
            return e!("Config load failed: {e}");
        }
    };
    let task = tokio::spawn(boot_oneport(listen));
    let abort = task.abort_handle();
    let api = tokio::spawn(async move {
        boot_api(api, abort).await;
    });
    // 即使api服务无法启动，也继续运行oneport服务
    task.await.unwrap_err(); // task正常情况下不会返回，除非发生了panic或者被取消，因此返回值一定是Err
    api.abort();
}

/// 启动oneport主服务, 默认监听 0.0.0.0:1111
async fn boot_oneport(listen: String) {
    i!("Starting oneport service on {listen}");
    let listener = TcpListener::bind(listen).await.unwrap();
    loop {
        let (visitor, addr) = match listener.accept().await {
            Ok(v) => v,
            Err(e) => unreachable!("{:?}", e),
        };
        i!("Request {addr} incoming");
        // Feature: 已有的会话不会在热重启时断开
        tokio::spawn(async move {
            serv(visitor, addr).await;
        });
    }
}

async fn serv(mut visitor: TcpStream, addr: SocketAddr) {
    visitor.readable().await.unwrap();
    let mut msg = vec![0; 1024];
    match visitor.try_read(&mut msg) {
        Ok(n) => {
            if n < 1 {
                i!("Request {addr} read EOF");
                return;
            }
            msg.truncate(n);
        }
        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
            i!("Request {addr} read would block");
        }
        Err(e) => {
            e!("Request {addr} read error: {e}");
            return;
        }
    }
    let head_msg = &msg[..if msg.len() > 10 { 10 } else { msg.len() }];
    i!("Request {addr} msg = {:?}({})", head_msg, String::from_utf8_lossy(head_msg));
    let rules = RULES.lock().await;
    let mut address = None;
    for (rule, target) in rules.as_slice() {
        if rule.len() <= msg.len() && rule == &msg[..rule.len()] {
            i!("Request {addr} matched: {target}");
            address = Some(target.clone());
            break;
        }
    }
    drop(rules);
    match address {
        None => return i!("Request {addr} not match"),
        Some(address) => {
            let mut stream = match TcpStream::connect(address).await {
                Ok(v) => v,
                Err(e) => return e!("Request {addr} serv error: {e}"),
            };
            stream.write_all(&msg).await.unwrap();
            #[cfg(feature = "dump")]
            let _ = dump_first(&addr.to_string(), &msg).await;
            let a = visitor.split();
            let b = stream.split();
            a2b::a2b(a, b).await;
            i!("Request {addr} finished");
        }
    }
}

#[allow(dead_code)]
async fn dump_first(who: &str, data: &[u8]) -> io::Result<()> {
    let time = get_time();
    let path = format!("{time} {who} first.txt");
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path.replace(":", "-"))
        .await?;
    let _ = file.write_all(data).await;
    Ok(())
}