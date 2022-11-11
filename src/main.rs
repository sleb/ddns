use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// domain to update
    domain: String,

    /// account token
    token: String,
}

fn main() {
    let cli = Cli::parse();
    let url = format!(
        "https://duckdns.org/update?domains={}&token={}&verbose=true",
        &cli.domain, &cli.token
    );

    match ureq::get(&url).call() {
        Ok(response) => {
            println!(
                "{}",
                response.into_string().unwrap_or_default().escape_debug()
            );
        }
        Err(e) => {
            eprintln!("Error: {}", e.to_string());
        }
    }
}
