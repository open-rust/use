use std::process::ExitStatus;

use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(version = "0.0.1", about = "进程守护")]
pub struct Param {
    /// 最大循环次数, 默认无穷
    #[arg(short, long, default_value_t = 0)]
    count: u64,

    /// 执行下一次命令的时间间隔(秒)
    #[arg(short, long, default_value_t = 5)]
    delay: u64,

    /// 执行每次命令的超时时间, 默认无穷(秒)
    #[arg(short, long, default_value_t = 0)]
    timeout: u64,

    /// 执行命令
    #[arg(required = true, num_args = 1..)]
    cmd: Vec<String>,
}

fn display_cmd(param: &Vec<String>) -> String {
    param
        .iter()
        .map(|it| {
            let it = it.replace("\"", "\\\"");
            if it.contains(" ") || it.contains("'") {
                format!("\"{it}\"")
            } else {
                it.clone()
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

pub async fn main(param: Param) -> tokio::io::Result<()> {
    macro_log::wtf!(param);
    let mut count = 0;
    let cmd = display_cmd(&param.cmd);
    loop {
        count += 1;
        if param.count != 0 && count > param.count {
            break;
        }
        macro_log::i!("执行命令: {}", cmd);
        // let _ = run(&param.cmd, param.timeout).await;
        let cmd = param.cmd.clone();
        let task = async move { run(&cmd, param.timeout).await };
        match tokio::spawn(task).await {
            Ok(Ok(Some(status))) => {
                macro_log::i!("程序退出, {status}");
            }
            Ok(Ok(None)) => {
                macro_log::i!("程序超时杀死");
            }
            Ok(Err(e)) => macro_log::e!("{e}"),
            Err(e) => macro_log::e!("{e}"),
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(param.delay)).await;
    }
    Ok(())
}

pub async fn run<'a>(
    cmd: &'a [impl AsRef<std::ffi::OsStr>],
    timeout: u64,
) -> tokio::io::Result<Option<ExitStatus>> {
    let mut exe = tokio::process::Command::new(&cmd[0]);
    exe.args(&cmd[1..]);
    let mut child = exe.spawn()?;
    let pid = child.id().unwrap_or_default();
    match timeout {
        0 => {
            let status = child.wait().await?;
            Ok(Some(status))
        }
        _ => {
            tokio::select! {
                Ok(status) = child.wait() => Ok(Some(status)),
                _ = tokio::time::sleep(tokio::time::Duration::from_secs(timeout)) => {
                    macro_log::i!("程序超时, pid: {}", pid);
                    let _ = child.kill().await?;
                    Ok(None)
                },
            }
        }
    }
}
