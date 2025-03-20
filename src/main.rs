use iplens::{cli, run};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::parse_args();
    run(args).await?;
    Ok(())
}
