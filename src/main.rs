use clap::{command, Parser};
use log::{error, info};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// domain to update
    domain: String,

    /// account token
    token: String,
}

fn main() {
    env_logger::init();

    let cli = Cli::parse();
    let url = format!(
        "https://duckdns.org/update?domains={}&token={}&verbose=true",
        &cli.domain, &cli.token
    );

    info!("waking up");
    match ureq::get(&url).call() {
        Ok(response) => {
            info!("{}", response.into_string().unwrap_or_default());
        }
        Err(e) => {
            error!("Error calling `{}`: {}", &url, e.to_string())
        }
    }
    info!("done");
}
