use crate::rmapi::endpoints;
use std::path::Path;
use uuid::Uuid;

struct Client {
    token: String,
    storage_url: Option<String>,
}

impl Client {
    /// Creates a new `Client` instance with the provided token.
    ///
    /// # Arguments
    ///
    /// * `token` - A `String` representing the authentication token.
    ///
    /// # Returns
    ///
    /// A new `Client` instance.
    pub async fn from_token(token: String) -> Client {
        Client {
            token: token,
            storage_url: None,
        }
    }

    /// Creates a new `Client` instance by obtaining a token.
    ///
    /// This function attempts to register a client and obtain a new token.
    ///
    /// # Returns
    ///
    /// A `Result` containing:
    /// - `Ok(Client)`: A new `Client` instance with the obtained token.
    /// - `Err(Error)`: An error if the token retrieval fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if the token retrieval process fails.
    pub async fn new() -> Result<Client, Error> {
        let token = endpoints::register_client().await?;
        return Ok(Client::from_token(token));
    }

    /// Refreshes the token for the current client.
    ///
    /// This function attempts to refresh the authentication token using the current token.
    ///
    /// # Returns
    ///
    /// A `Result` containing:
    /// - `Ok(())`: If the token refresh was successful.
    /// - `Err(Error)`: An error if the token refresh fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if the token refresh process fails.
    pub async fn refresh_token() -> Result<(), Error> {
        self.token = endpoints::refresh_token(self.token).await?;
        return Ok(());
    }

    pub async fn save_token(path: Path) -> Result<(), Error> {
        let mut file = File::create(path)?;
        file.write_all(self.token.as_bytes())?;
        return Ok(());
    }

    pub async fn get_storage_url(&mut self) -> Result<String, Error> {
        let client = reqwest::Client::new();
        let response = client
            .get(formatcp!("{STORAGE_API_URL}/"))
            .header("Authorization", formatcp!("Bearer {self.token}"))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("User-Agent", "google-chrome")
            .send()
            .await?;

        let storage_url: String = response.json().await?;
        self.storage_url = Some(storage_url);
        return Ok(storage_url);
    }

    pub async fn get_collection(&self, id: Uuid) -> Result<Collection, Error> {
        let client = reqwest::Client::new();
        let response = client
            .get(formatcp!("{STORAGE_API_URL}/{id}"))
            .header("Authorization", formatcp!("Bearer {self.token}"))
            .send()
            .await?;

        let collection: Collection = response.json().await?;
        return Ok(collection);
    }

    pub async fn get_document(&self, id: Uuid) -> Result<Document, Error> {
        let client = reqwest::Client::new();
        let response = client
            .get(formatcp!("{STORAGE_API_URL}/{id}"))
            .header("Authorization", formatcp!("Bearer {self.token}"))
            .send()
            .await?;

        let document: Document = response.json().await?;
        return Ok(document);
    }
}
