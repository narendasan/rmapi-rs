use clap::Parser;

use rmapi::endpoints;
use rmapi::error::Error;

#[derive(Parser, Debug)]
struct Args {
    #[arg(
        short,
        long,
        help = "Token to register this device. Go to https://my.remarkable.com/device/desktop/connect to get a new token."
    )]
    token: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();
    let token = endpoints::register_client(&args.token).await?;
    println!("Token: {}", token);
    let new_token = endpoints::refresh_token(&token).await?;
    println!("Token: {}", new_token);
    Ok(())
}
