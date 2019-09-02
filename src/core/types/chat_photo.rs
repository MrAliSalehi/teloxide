use serde::Deserialize;

#[derive(Debug, Deserialize, Hash, PartialEq, Eq)]
pub struct ChatPhoto {
    pub small_file_id: String,
    pub big_file_id: String,
}
