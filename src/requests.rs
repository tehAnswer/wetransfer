extern crate serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTransferRequest {
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub files: Vec<FileRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileRequest {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub size: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUrlRequest {}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CompleteFileUploadRequest {
    pub part_numbers: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalizeRequest {}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBoardRequest {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
} 
