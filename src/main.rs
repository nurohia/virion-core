
mod crypto;
mod protocol;
mod transport;

use clap::Parser;
use std::fs;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value = "/etc/virion/rules.yaml")]
    config: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let config_content = fs::read_to_string(&args.config)?;
    println!("加载配置: {}", args.config);
    transport::router::run(config_content).await?;
    Ok(())
}
