use clap::Parser;
use serde_derive::{Deserialize, Serialize};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

#[derive(Debug, Serialize, Deserialize)]
struct Affirm {
    affirmation: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args = Args::parse();
    let url = "https://www.affirmations.dev/";
    for _ in 0..args.count {
        let affirm: Affirm = reqwest::Client::new().get(url).send().await?.json().await?;
        println!("{:#?}", affirm.affirmation);
    }
    Ok(())
}
