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
pub struct Transfer {
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
    pub files: Vec<File>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct File {
    #[serde(default)]
    pub multipart: Multipart,
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
pub struct Multipart {
    #[serde(default)]
    pub part_numbers: u64,
    #[serde(default)]
    pub chunk_size: u64,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct GetUploadUrlResponse {
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub success: bool
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CompleteFileUploadResponse {
    pub id: String,
    pub retries: u64,
    pub name: String,
    pub size: u64,
    pub chunk_size: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateBoardResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub state: String,
    pub url: String,
    pub items: Vec<::serde_json::Value>,
}




