use serde::{Serialize, Deserialize};
use crate::rmapi::object::RemarkableObject;

#[derive(Debug)]
enum CollectionType {

}

#[derive(Serialize, Deserialize, Debug)]
struct Collection {
    id: Uuid,
    version: u64,
    message: String,
    blob_url_get: String,
    blob_url_get_expires: DateTime<Utc>,
    blob_url_put: String,
    blob_url_put_expires: DateTime<Utc>,
    last_modified: DateTime<Utc>,
    display_name: String,
    current_page: u64,
    type: CollectionType,
    parent: String,
    objects: Vec<RemarkableObject>,
}

impl RemarkableObject for Collection {
    fn register_client(code: String) -> Result<String, Error> {
        unimplemented!()
    }
}
