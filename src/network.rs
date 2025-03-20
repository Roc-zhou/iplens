use pnet::datalink;
use reqwest;
use serde_json::Value;
use std::error::Error;
use std::net::IpAddr;

/// 获取本地 IP 地址（过滤回环地址）
pub fn get_local_ips() -> Result<Vec<(String, IpAddr)>, Box<dyn Error>> {
    let mut ips = Vec::new();

    for interface in datalink::interfaces() {
        let interface_name = interface.name.clone();
        for ip in interface.ips {
            match ip.ip() {
                IpAddr::V4(ipv4) if !ipv4.is_loopback() => {
                    ips.push((interface_name.clone(), IpAddr::V4(ipv4)));
                }
                IpAddr::V6(ipv6) if !ipv6.is_loopback() => {
                    ips.push((interface_name.clone(), IpAddr::V6(ipv6)));
                }
                _ => {}
            }
        }
    }

    // 优先排序 IPv4 地址
    ips.sort_by(|a, b| match (a.1, b.1) {
        (IpAddr::V4(_), IpAddr::V6(_)) => std::cmp::Ordering::Less,
        (IpAddr::V6(_), IpAddr::V4(_)) => std::cmp::Ordering::Greater,
        _ => a.0.cmp(&b.0),
    });

    Ok(ips)
}

/// 获取公网 IP 地址（尝试多个 API 端点）
pub async fn get_public_ip() -> Result<String, Box<dyn Error>> {
    let api_endpoints = [
        "https://api.ipify.org?format=json",
        "https://api.ip.sb/jsonip",
        "https://api.myip.com",
    ];

    for endpoint in api_endpoints {
        match reqwest::get(endpoint).await {
            Ok(response) => {
                if let Ok(json) = response.json::<Value>().await {
                    if let Some(ip) = json["ip"].as_str() {
                        return Ok(ip.to_string());
                    }
                }
            }
            Err(_) => continue,
        }
    }

    Err("无法获取公网 IP 地址".into())
}

/// 获取 DNS 服务器地址
pub fn get_dns_servers() -> Result<Vec<String>, Box<dyn Error>> {
    #[cfg(target_os = "windows")]
    {
        // Windows 平台使用 WMI 查询 DNS 服务器
        use serde::Deserialize;
        use wmi::{COMLibrary, WMIConnection};

        #[derive(Deserialize, Debug)]
        struct DNSServer {
            DNSServerSearchOrder: Option<Vec<String>>,
        }

        let com_lib = COMLibrary::new()?;
        let wmi_con = WMIConnection::new(com_lib)?;
        let dns_servers: Vec<DNSServer> = wmi_con.query()?;

        for server in dns_servers {
            if let Some(servers) = server.DNSServerSearchOrder {
                return Ok(servers);
            }
        }

        Ok(Vec::new())
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        // Linux/macOS 平台读取 /etc/resolv.conf 文件
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let file = match File::open("/etc/resolv.conf") {
            Ok(file) => file,
            Err(_) => return Ok(Vec::new()),
        };

        let reader = BufReader::new(file);
        let mut dns_servers = Vec::new();

        for line in reader.lines() {
            let line = line?;
            if line.starts_with("nameserver") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    dns_servers.push(parts[1].to_string());
                }
            }
        }

        Ok(dns_servers)
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    {
        Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_ip_filter() {
        let ips = get_local_ips().unwrap();
        for (_, ip) in &ips {
            match ip {
                IpAddr::V4(ipv4) => assert!(!ipv4.is_loopback(), "不应包含回环地址"),
                IpAddr::V6(ipv6) => assert!(!ipv6.is_loopback(), "不应包含回环地址"),
            }
        }
    }
}
