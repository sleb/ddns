use std::{thread, time::Duration};

use clap::{command, Parser};
use log::{error, info};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// domain to update
    domain: String,

    /// account token
    token: String,

    /// number minutes to sleep (triggers daemon mode)
    #[arg(short, long)]
    sleep: Option<u32>,
}

fn update(domain: &str, token: &str) {
    let url = format!(
        "https://duckdns.org/update?domains={}&token={}&verbose=true",
        domain, token
    );

    match ureq::get(&url).call() {
        Ok(response) => {
            info!("{}", response.into_string().unwrap_or_default());
        }
        Err(e) => {
            error!("Error calling `{}`: {}", &url, e.to_string())
        }
    }
}

fn main() {
    env_logger::init();

    let cli = Cli::parse();

    loop {
        info!("waking up");
        update(&cli.domain, &cli.token);
        match cli.sleep {
            Some(minutes) => {
                info!("sleeping {} minutes...", minutes);
                thread::sleep(Duration::from_secs(minutes as u64 * 60u64))
            }
            None => break,
        }
        info!("done");
    }
}
