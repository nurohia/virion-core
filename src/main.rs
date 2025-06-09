mod config;
mod crypto;
mod protocol;
mod transport;

use clap::Parser;
use config::{Config, Mode};

#[derive(Parser)]
#[command(author, version, about = "Virion Core - Secure Tunnel Engine", long_about = None)]
struct Cli {
    #[arg(short, long, default_value = "config/client.yaml")]
    config: String,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let cli = Cli::parse();

    let config = match Config::from_file(&cli.config) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("配置加载失败: {}", e);
            std::process::exit(1);
        }
    };

    match config.mode {
        Mode::Client => transport::tcp::start_tcp_listener(&config).await.unwrap(),
        Mode::Relay => println!("TODO: relay"),
        Mode::Server => transport::tcp::start_tcp_listener(&config).await.unwrap(),
    }
}
