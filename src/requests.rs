extern crate serde_json;

#[derive(Debug, Serialize, Deserialize)]
/// represents the body schema required to initiate a transfer request.
pub struct CreateTransferRequest {
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub files: Vec<FileRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
/// as a part of `CreateTransferRequest`, it represents the files to be transfered.
pub struct FileRequest {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub size: u64,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
/// represents the body schema required to mark a file upload as completed.
pub struct CompleteFileUploadRequest {
    pub part_numbers: u64,
}

#[derive(Debug, Serialize, Deserialize)]
/// represents the body schema required to mark a transfer as finalized.
pub struct FinalizeRequest {}

#[derive(Debug, Serialize, Deserialize)]
/// represents the body schema required to create a board.
pub struct CreateBoardRequest {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
/// represents the body schema required to add a link to a board.
pub struct AddLink {
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
/// represents the body schema required to mark a file upload for a board as completed.
pub struct CompleteFileBoardUploadRequest {}
