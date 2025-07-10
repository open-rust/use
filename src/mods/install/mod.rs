#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use clap::Parser;

use crate::utils::docker::is_running_in_docker;

#[derive(Parser, Debug, Clone)]
#[command(version, about = "安装到系统目录")]
pub struct Param {
    #[cfg(unix)]
    #[arg(default_value_t = String::from("/usr/local/bin"))]
    dest: String,

    #[cfg(windows)]
    #[arg(default_value_t = String::from("C:\\Windows"))]
    dest: String,
}

pub async fn main(param: Param) -> tokio::io::Result<()> {
    let is_docker = is_running_in_docker();
    let exe = std::env::current_exe()?;
    let file = tokio::fs::read(exe).await?;
    let mut dest = format!(
        "{}{}use{}",
        &param.dest,
        std::path::MAIN_SEPARATOR,
        if cfg!(windows) { ".exe" } else { "" }
    );
    if is_docker {
        dest = format!("/host{}", dest);
    }
    tokio::fs::write(&dest, &file).await?;
    #[cfg(unix)]
    {
        let mut perms = tokio::fs::metadata(&dest).await?.permissions();
        let mode = perms.mode();
        perms.set_mode(mode | 0b001001001); // chmod +x
        tokio::fs::set_permissions(&dest, perms).await?;
    }
    macro_log::i!("Installed: {}", dest);
    Ok(())
}
