extern crate serde_json;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct WeTransferError {
    #[serde(default)]
    pub status: u16,
    #[serde(default)]
    pub message: String,

}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Login {
    pub success: bool,
    #[serde(default)]
    pub token: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct CreateTransferResponse {
    #[serde(default)]
    pub success: bool,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub state: String,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub expires_at: String,
    #[serde(default)]
    pub files: Vec<FileResponse>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct FileResponse {
    #[serde(default)]
    pub multipart: MultipartResponse,
    #[serde(default)]
    pub size: i64,
    #[serde(default, rename = "type")]
    pub file_type: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub id: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct MultipartResponse {
    #[serde(default)]
    pub part_numbers: i64,
    #[serde(default)]
    pub chunk_size: i64,
}
