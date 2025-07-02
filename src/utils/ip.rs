/// 获取接口IP地址
pub fn get_interface_ipv4s() -> Vec<String> {
    let ifaces = ifcfg::IfCfg::get().expect("could not get interfaces");
    let mut ips = ifaces
        .iter()
        .map(|it| {
            // let name = it.name.to_string();
            let ip = it
                .addresses
                .iter()
                .map(|it| {
                    if let ifcfg::AddressFamily::IPv4 = it.address_family {
                        it.address
                            .iter()
                            .map(|it| it.ip().to_string())
                            .collect::<Vec<String>>()
                            .pop()
                            .unwrap_or_default()
                    } else {
                        "".to_string()
                    }
                })
                .filter(|it| it != "")
                .collect::<Vec<String>>()
                .join(",");
            ip
        })
        .filter(|it| it != "")
        .collect::<Vec<String>>();
    ips.sort();
    ips
}

#[test]
fn test_get_interfaces_and_ipv4s() {
    let ip = get_interface_ipv4s();
    macro_log::wtf!(ip);
}

/// 获取公网IP地址
pub fn get_public_ip() -> String {
    let ip = "0.0.0.0".to_string();
    let mut box_ip = Box::new(ip);
    let apis = vec!["ifconfig.me", "ip.sb"];
    let _ = apis.iter().try_for_each(|it| -> std::io::Result<()> {
        let addr = std::net::ToSocketAddrs::to_socket_addrs(&format!("{}:80", it));
        if addr.is_err() {
            return Ok(());
        }
        let addr = addr.unwrap().next().unwrap();
        let timeout = std::time::Duration::from_millis(5000);
        match std::net::TcpStream::connect_timeout(&addr, timeout) {
            Ok(mut stream) => {
                let msg = format!(
                    "GET / HTTP/1.1\r\nHost: {}\r\nUser-Agent: curl/8.9.1\r\n\r\n",
                    it
                );
                let msg = msg.as_bytes();
                std::io::Write::write(&mut stream, msg).unwrap();
                let mut buf = [0u8; 1024];
                let le = std::io::Read::read(&mut stream, &mut buf).unwrap();
                let resp = std::str::from_utf8(&buf[0..le]).unwrap();
                let resp = resp.split("\r\n\r\n").nth(1).unwrap().trim();
                box_ip = Box::new(resp.into());
                return Err(std::io::ErrorKind::Interrupted.into()); // interrupt the loop
            }
            _ => (),
        }
        Ok(())
    });
    *box_ip
}

#[test]
fn test_get_public_ip() {
    let ip = get_public_ip();
    macro_log::wtf!(ip);
}
