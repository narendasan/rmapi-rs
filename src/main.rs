use clap::Parser;

mod rmapi;
use crate::rmapi::endpoints;
use crate::rmapi::error::Error;

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
    let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJhdXRoMC11c2VyaWQiOiJhdXRoMHw1YWJlOTMyYWM0YTBmOTA3YmU1NWU4MTAiLCJkZXZpY2UtZGVzYyI6ImRlc2t0b3Atd2luZG93cyIsImRldmljZS1pZCI6IjY0YmY2MWIyLTZkMDItNDZhYS05NWE2LWU5NDdlMTY2YzZiNCIsImlhdCI6MTcyNjEwMjM2OCwiaXNzIjoick0gV2ViQXBwIiwianRpIjoiY2swdFd1YzRnSVU9IiwibmJmIjoxNzI2MTAyMzY4LCJzdWIiOiJyTSBEZXZpY2UgVG9rZW4ifQ.JMuzWoPqcMnf9er4NTVq8o-2l7lnm9O59BwkCatfP24
".to_string(); //endpoints::register_client(&args.token).await?;
    println!("Token: {}", token);
    let new_token = endpoints::refresh_token(&token).await?;
    println!("Token: {}", new_token);
    Ok(())
}
