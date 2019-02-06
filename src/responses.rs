extern crate serde_json;

#[derive(Default, Debug, Serialize, Deserialize)]
/// represents an error. Used as the root error type of the crate.
pub struct WeTransferError {
    #[serde(default)]
    pub status: u16,
    #[serde(default)]
    pub message: String,

}

#[derive(Default, Debug, Serialize, Deserialize)]
/// represents the response from a login attempt.
pub struct Login {
    pub success: bool,
    #[serde(default)]
    pub token: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
/// represents a WeTransfer file transfer. 
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
/// represents a file within a Transfer.
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
/// represents the response when requesting the presigned S3 upload url.
pub struct GetUploadUrlResponse {
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub success: bool
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
/// represents the response of completing a file upload.
pub struct CompleteFileUploadResponse {
    pub id: String,
    pub retries: u64,
    pub name: String,
    pub size: u64,
    pub chunk_size: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
/// represents a Board, where link and files can be added.
pub struct Board {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub state: String,
    pub url: String,
    pub items: Vec<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
/// represents a Board item which displays an external website.
pub struct Link {
    pub id: String,
    pub url: String,
    pub meta: Meta,
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Meta {
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
/// represents a Board item which renders a traditional file (e.g image).
pub struct FileBoard {
    pub id: String,
    pub name: String,
    pub size: u64,
    pub multipart: MultipartFileBoard,
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MultipartFileBoard {
    pub id: String,
    pub part_numbers: u64,
    pub chunk_size: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompleteFileBoardUploadResponse {
    pub success: bool,
    pub message: String,
}




