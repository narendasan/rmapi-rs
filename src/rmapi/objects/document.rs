use serde::{Serialize, Deserialize};
use crate::rmapi::object::RemarkableObject;

#[derive(Debug)]
enum DocumentType {

}

#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    id: Uuid,
    version: u64,
    message: String,
    success: bool,
    blob_url_get: String,
    blob_url_put: String,
    blob_url_put_expires: DateTime<Utc>,
    last_modified: DateTime<Utc>,
    display_name: String,
    current_page: u64,
    bookmarked: bool,
    type: DocumentType,
    parent: String,
}

impl Document {
    pub async fn new(token: String, id: Uuid) -> Result<Document, Error> {
        let client = reqwest::Client::new();
        let response = client
            .get(formatcp!("{STORAGE_API_URL}/{id}"))
            .header("Authorization", formatcp!("Bearer {token}"))
            .send()
            .await?;

        let document: Document = response.json().await?;
        return Ok(document);
    }

    pub async fn save(&self) -> Result<(), Error> {
        let client = reqwest::Client::new();
        let response = client
            .put(formatcp!("{STORAGE_API_URL}/{self.id}"))
            .header("Authorization", formatcp!("Bearer {self.token}"))
            .json(&self)
            .send()
            .await?;

        return Ok(());
    }

    pub async fn delete(&self) -> Result<(), Error> {
        let client = reqwest::Client::new();
        let response = client
            .delete(formatcp!("{STORAGE_API_URL}/{self.id}"))
            .header("Authorization", formatcp!("Bearer {self.token}"))
            .send()
            .await?;

        return Ok(());
    }
}

impl RemarkableObject for Document {
    fn register_client(code: String) -> Result<String, Error> {
        unimplemented!()
    }
}
