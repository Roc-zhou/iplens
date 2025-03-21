// use crate::vpn::VpnStatus;
use prettytable::{row, Table};
use serde_json::{json, to_string_pretty};
use std::error::Error;
use std::net::IpAddr;

/// 以表格格式输出 IP 信息
pub fn print_table(
    timestamp: &str,
    local_ips: &[(String, IpAddr)],
    // public_ip: &str,
    // vpn_status: &VpnStatus,
    // dns_servers: &[String],
) -> Result<(), Box<dyn Error>> {
    let mut table = Table::new();

    table.add_row(row!["时间戳", timestamp]);

    // 本地 IP
    if !local_ips.is_empty() {
        let mut local_ip_str = String::new();
        for (interface, ip) in local_ips {
            local_ip_str.push_str(&format!("{}: {}\n", interface, ip));
        }
        table.add_row(row!["本地 IP", local_ip_str.trim_end()]);
    } else {
        table.add_row(row!["本地 IP", "未检测到"]);
    }

    // // 公网 IP
    // table.add_row(row!["公网 IP", public_ip]);

    // // VPN 状态
    // let vpn_icon = if vpn_status.connected { "✅" } else { "❌" };

    // let vpn_info = if vpn_status.connected {
    //     format!(
    //         "{} 已连接 ({}): {}",
    //         vpn_icon,
    //         vpn_status
    //             .interface_name
    //             .as_ref()
    //             .unwrap_or(&"未知".to_string()),
    //         vpn_status
    //             .ip
    //             .unwrap_or(IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0)))
    //     )
    // } else {
    //     format!("{} 未连接", vpn_icon)
    // };

    // table.add_row(row!["VPN 状态", vpn_info]);

    // // DNS 服务器
    // if !dns_servers.is_empty() {
    //     let dns_str = dns_servers.join("\n");
    //     table.add_row(row!["DNS 服务器", dns_str]);
    // } else {
    //     table.add_row(row!["DNS 服务器", "未检测到"]);
    // }

    table.printstd();
    Ok(())
}

/// 以 JSON 格式输出 IP 信息
pub fn print_json(
    timestamp: &str,
    local_ips: &[(String, IpAddr)],
    // public_ip: &str,
    // vpn_status: &VpnStatus,
    // dns_servers: &[String],
) -> Result<(), Box<dyn Error>> {
    let local_ip_map = local_ips
        .iter()
        .map(|(interface, ip)| (interface.clone(), ip.to_string()))
        .collect::<Vec<_>>();

    // let vpn_info = if vpn_status.connected {
    //     json!({
    //         "connected": true,
    //         "interface": vpn_status.interface_name,
    //         "ip": vpn_status.ip.map(|ip| ip.to_string())
    //     })
    // } else {
    //     json!({
    //         "connected": false
    //     })
    // };

    let json_output = json!({
        "timestamp": timestamp,
        "local_ips": local_ip_map,
        // "public_ip": public_ip,
        // "vpn": vpn_info,
        // "dns_servers": dns_servers
    });

    println!("{}", to_string_pretty(&json_output)?);
    Ok(())
}
