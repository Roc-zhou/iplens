use pnet::datalink;
use std::error::Error;
use std::net::IpAddr;

/// VPN 状态信息
#[derive(Debug)]
pub struct VpnStatus {
    pub connected: bool,
    pub interface_name: Option<String>,
    pub ip: Option<IpAddr>,
}

/// 检查 VPN 连接状态
pub fn check_vpn_status() -> Result<VpnStatus, Box<dyn Error>> {
    let interfaces = datalink::interfaces();

    // VPN 接口名称模式
    let vpn_patterns = [
        "tun", "tap", "ppp", "vpn", "utun", "ipsec", "wg", "nordlynx", "proton",
    ];

    for interface in interfaces {
        let name = interface.name.to_lowercase();
        if vpn_patterns.iter().any(|&pattern| name.contains(pattern)) {
            // 找到 VPN 接口
            if !interface.ips.is_empty() {
                let ip = interface.ips[0].ip();
                return Ok(VpnStatus {
                    connected: true,
                    interface_name: Some(interface.name),
                    ip: Some(ip),
                });
            }
        }
    }

    // 未找到 VPN 接口
    Ok(VpnStatus {
        connected: false,
        interface_name: None,
        ip: None,
    })
}
