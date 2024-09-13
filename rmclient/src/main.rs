use clap::Parser;
use std::path::Path;

use dirs::cache_dir;
use rmapi::Client;
use std::path::PathBuf;
use std::process;

mod rmclient;
use crate::rmclient::error::Error;
use tokio::fs::File;

pub fn default_token_file_path() -> PathBuf {
    cache_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("rmapi/auth_token")
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(
        short = 'c',
        long,
        help = "code to register this client with reMarkable. Go to https://my.remarkable.com/device/desktop/connect to get a new code."
    )]
    code: Option<String>,

    #[arg(
        short = 't',
        long = "auth-token-file",
        help = "Path to the file that holds a previously generated session token",
        default_value = default_token_file_path().into_os_string()
    )]
    auth_token_file: PathBuf,
}

async fn write_token_file(auth_token: &str, auth_token_file: &Path) -> Result<(), Error> {
    if let Some(parent) = auth_token_file.parent() {
        log::debug!("Making client cache dir {:?}", parent);
        tokio::fs::create_dir_all(parent).await?;
    }
    tokio::fs::write(auth_token_file, auth_token).await?;
    log::debug!("Saving auth token to: {:?}", auth_token_file);
    Ok(())
}

async fn refresh_client_token(client: &mut Client, auth_token_file: &Path) -> Result<(), Error> {
    client.refresh_token().await?;
    log::debug!("Saving new auth token to: {:?}", auth_token_file);
    write_token_file(&client.auth_token, auth_token_file).await?;
    Ok(())
}

/// Creates a new `Client` instance from a token stored in a file.
///
/// # Arguments
///
/// * `auth_token_file` - A `&Path` pointing to the file containing the authentication token.
///
/// # Returns
///
/// A `Result` containing:
/// - `Ok(Client)`: A new `Client` instance with the token read from the file.
/// - `Err(Error)`: An error if the token file is not found, invalid, or cannot be read.
///
/// # Errors
///
/// This function will return an error if:
/// - The token file does not exist (`Error::TokenFileNotFound`).
/// - The token file is not a regular file (`Error::TokenFileInvalid`).
/// - Reading the token file fails.
/// - Creating a new `Client` from the read token fails.
async fn client_from_token_file(auth_token_file: &Path) -> Result<Client, Error> {
    if !auth_token_file.exists() {
        Err(Error::TokenFileNotFound)
    } else if !auth_token_file.is_file() {
        Err(Error::TokenFileInvalid)
    } else {
        let auth_token = tokio::fs::read_to_string(&auth_token_file).await?;
        log::debug!(
            "Using token from {:?} to create a new client",
            auth_token_file
        );
        Ok(Client::from_token(&auth_token).await?)
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    let args = Args::parse();

    let mut client;

    if let Some(code) = args.code {
        client = Client::new(&code).await?;
    } else if args.auth_token_file.exists() {
        client = client_from_token_file(&args.auth_token_file).await?;
    } else {
        eprintln!("No token file found at {:?}, please either correct the path with `-t` or provide a new verification code with `-c`", args.auth_token_file);
        process::exit(1);
    }

    println!("Client token: {:?}", client.auth_token);
    println!("Storage url: {:?}", client.storage_url);
    //client.sync_root().await?;

    let file_path = Path::new("test2.pdf");
    let file = File::open(file_path).await?;
    client.upload_file(file).await?;

    Ok(())
}
