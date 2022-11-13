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
    sleep: Option<u64>,

    /// verbosity - can be specified multiple times
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: Option<u8>,
}

fn update(domain: &str, token: &str) {
    let url = format!(
        "https://duckdns.org/update?domains={}&token={}&verbose=true",
        domain, token
    );

    match ureq::get(&url).call() {
        Ok(response) => {
            info!("{}", response.into_string().unwrap_or(String::from("")));
        }
        Err(e) => {
            error!("Error calling `{}`: {}", &url, e.to_string())
        }
    }
}

fn main() {
    let cli = Cli::parse();
    stderrlog::new()
        .module(module_path!())
        .verbosity(cli.verbose.unwrap_or(0) as usize + 1)
        .init()
        .unwrap();

    loop {
        info!("waking up");
        update(&cli.domain, &cli.token);
        info!("done");
        match cli.sleep {
            Some(minutes) => {
                info!("sleeping {} minutes...", minutes);
                thread::sleep(Duration::from_secs(minutes * 60))
            }
            None => break,
        }
    }
}
