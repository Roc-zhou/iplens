pub mod cli;
pub mod network;
pub mod output;
pub mod vpn;

use cli::Args;
use network::{get_dns_servers, get_local_ips, get_public_ip};
use output::{print_json, print_table};
use vpn::check_vpn_status;

/// 运行 iplens 工具的主函数
pub async fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    // 获取当前时间戳
    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // 获取各种 IP 信息
    let local_ips = get_local_ips()?;
    let public_ip = get_public_ip().await?;
    let vpn_status = check_vpn_status()?;
    let dns_servers = get_dns_servers()?;

    // 根据参数选择输出格式
    if args.json {
        print_json(
            &timestamp,
            &local_ips,
            &public_ip,
            &vpn_status,
            &dns_servers,
        )?;
    } else {
        print_table(
            &timestamp,
            &local_ips,
            &public_ip,
            &vpn_status,
            &dns_servers,
        )?;
    }

    Ok(())
}
