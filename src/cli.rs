use clap::Parser;

/// IP 信息检测工具
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// 以 JSON 格式输出结果
    #[clap(long)]
    pub json: bool,
}

/// 解析命令行参数
pub fn parse_args() -> Args {
    Args::parse()
}
