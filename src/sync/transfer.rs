use sync::requester::RequestService;
use std::fs;
use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use std::borrow::ToOwned;

use responses::*;
use requests::*;

#[derive(Debug)]
pub struct TransferService {
    requester: RequestService,
}

#[cfg(not(test))]
const TRANSFERS_URL: &'static str = "https://dev.wetransfer.com/v2/transfers";
#[cfg(test)]
const TRANSFERS_URL: &'static str = mockito::SERVER_URL;

impl TransferService {
    pub fn new<S: Into<String>+ToString>(jwt: S, app_token: S) -> TransferService {
        TransferService {
            requester: RequestService::new(jwt.to_string(), app_token.to_string(), TRANSFERS_URL.to_owned())
        }
    }

    pub fn find<S: Into<String>+ToString>(&self, transfer_id: S) -> Result<Transfer, WeTransferError> {
        let path = format!("/{}", transfer_id.to_string());
        self.requester.get::<Transfer>(&path)
    }

    pub fn create<S: Into<String>+ToString>(&self, message: S, paths: &Vec<S>) -> Result<Transfer, WeTransferError> {
        let transfer = self.create_transfer_request(message, paths)?;
        for (index, file) in transfer.files.iter().enumerate() {
            let path_str = paths.get(index).unwrap().to_string();
            let path = Path::new(path_str.as_str());
            let mut file_io = File::open(path).unwrap();
            let mut buffer = std::vec::from_elem(0, file.multipart.chunk_size as usize);
            for part in 1..=file.multipart.part_numbers {
                file_io.read(&mut buffer).unwrap();
                let s3_url = self.upload_url_for(transfer.id.to_owned(), file.id.to_owned(), part)?.url;
                self.requester.file_upload(s3_url, &buffer).unwrap();
            }
            self.mark_as_complete(transfer.id.to_owned(), file.id.to_owned(), file.multipart.part_numbers)?;
        }
        self.finalize(transfer.id.to_owned())
    }

    pub fn finalize<S: Into<String>+ToString>(&self, transfer_id: S) -> Result<Transfer, WeTransferError> {
        let path = format!("/{}/finalize", transfer_id.to_string());
        self.requester.put::<FinalizeRequest, Transfer>(&path, FinalizeRequest {})
    }

    pub fn upload_url_for<S: Into<String>+ToString>(&self, upload_id: S, file_id: S, part: u64) -> Result<GetUploadUrlResponse, WeTransferError> {
        let path = format!("/{}/files/{}/upload-url/{}", upload_id.to_string(), file_id.to_string(), part);
        self.requester.get::<GetUploadUrlResponse>(&path)
    }

    pub fn create_transfer_request<S: Into<String>+ToString>(&self, message: S, paths: &Vec<S>) -> Result<Transfer, WeTransferError> {
        let files = paths.into_iter().map(|path_str: &S| {
            // Compiler suggested to put this expression under
            // a let variable.
            let path_str_ref = path_str.to_string();
            let path = Path::new(&path_str_ref);
            self.extract_file_info(&path)
        }).collect();

        match files {
            Ok(file_requests) => {
                let payload = CreateTransferRequest {
                    message: message.to_string(),
                    files: file_requests
                };

                self.requester.post::<CreateTransferRequest, Transfer>("", payload)
            },
            Err(transfer_error) => Err(transfer_error)
        }
    }

    pub fn mark_as_complete<S: Into<String>+ToString>(&self, upload_id: S, file_id: S, part_numbers: u64) -> Result<CompleteFileUploadResponse, WeTransferError> {
        let payload = CompleteFileUploadRequest { part_numbers: part_numbers };
        let path = format!("/{}/files/{}/upload-complete", upload_id.to_string(), file_id.to_string());
        self.requester.put::<CompleteFileUploadRequest, CompleteFileUploadResponse>(&path, payload)
    } 

    fn extract_file_info(&self, path: &Path) -> Result<FileRequest, WeTransferError> {
        match fs::metadata(path) {
            Ok(io) => Ok(FileRequest {
                // Safe to unwrap path after a successful read
                name: path.file_name().unwrap().to_str().unwrap().to_string(),
                size: io.len()
            }),
            Err(error) => Err(WeTransferError {
                status: 0,
                message: error.description().to_string(),
            })
        }
    }   
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;
    use std::fs;

    #[test]
    fn it_creates_transfers() {
        let body = fs::read_to_string(Path::new("src/support/create_transfer_request.json")).expect("Fixtures:");
        let _m = mock("POST", "/")
          .with_status(201)
          .match_header("Authorization", "Bearer jwt-token")
          .match_header("x-api-key", "1234")
          .with_body(body)
          .create();

        let service = TransferService::new("jwt-token", "1234");
        let transfer_request = service.create_transfer_request("foo", &vec!["Cargo.toml"]).unwrap();
        assert!(transfer_request.success);
        assert_eq!(transfer_request.id, "32a6ef6003f1429be0cf1674dd8fbdef20181019143517");
        assert_eq!(transfer_request.message, "foo");
        assert_eq!(transfer_request.state, "uploading");
        assert_eq!(transfer_request.url, None);
        assert_eq!(transfer_request.files[0].multipart.part_numbers, 1);
        assert_eq!(transfer_request.files[0].multipart.chunk_size, 212);
        assert_eq!(transfer_request.files[0].file_type, "file");
        assert_eq!(transfer_request.files[0].name, "Cargo.toml");
        assert_eq!(transfer_request.files[0].id, "c964caf6c54343f3b6e9610cb4ac5ea220181019143517");
    }

    #[test]
    fn it_requests_s3_urls_for_uploading_parts() {
        let upload_id = "041bae61-adb4-4ba2-80eb-48719396f0e3";
        let file_id = "783b21e9-f8f1-46d5-894c-62954109d11d";
        let path = format!("/{}/files/{}/upload-url/1", upload_id, file_id);
        let _m = mock("GET", path.as_str())
          .with_status(201)
          .match_header("Authorization", "Bearer jwt-token")
          .match_header("x-api-key", "1234")
          .with_body("{\"success\": true, \"url\":\"https://s3-wetransfer.com/uploadhere\"}")
          .create();

        let service = TransferService::new("jwt-token", "1234");
        let upload_url_request = service.upload_url_for(upload_id, file_id, 1).unwrap();

        assert!(upload_url_request.success);
        assert_eq!(upload_url_request.url, "https://s3-wetransfer.com/uploadhere");
    }


    #[test]
    fn it_completes_file_uploads() {
        let body = fs::read_to_string(Path::new("src/support/complete_file_upload.json")).expect("Fixtures:");
        let upload_id = "041bae61-adb4-4ba2-80eb-48719396f0e3";
        let file_id = "783b21e9-f8f1-46d5-894c-62954109d11d";
        let path = format!("/{}/files/{}/upload-complete", upload_id, file_id);
        let _m = mock("PUT", path.as_str())
          .with_status(200)
          .match_body("{\"part_numbers\":1}")
          .match_header("Authorization", "Bearer jwt-token")
          .match_header("x-api-key", "1234")
          .with_body(body)
          .create();

        let service = TransferService::new("jwt-token", "1234");
        let result = service.mark_as_complete(upload_id, file_id, 1);
        assert!(result.is_ok());
    }

    #[test]
    fn it_finalizes_transfers() {
        let body = fs::read_to_string(Path::new("src/support/finalize_response.json")).expect("Fixtures:");
        let upload_id = "041bae61-adb4-4ba2-80eb-48719396f0e3";
        let path = format!("/{}/finalize", upload_id);
        let _m = mock("PUT", path.as_str())
          .with_status(200)
          .match_header("Authorization", "Bearer jwt-token")
          .match_header("x-api-key", "1234")
          .with_body(body)
          .create();

        let service = TransferService::new("jwt-token", "1234");
        let transfer = service.finalize(upload_id).unwrap();
        assert_eq!(transfer.id, upload_id);
        assert_eq!(transfer.url.unwrap(), "https://we.tl/t-12344657");
        assert_eq!(transfer.state, "processing");
    }

    #[test]
    fn it_finds_transfers_by_id() {
        let body = fs::read_to_string(Path::new("src/support/finalize_response.json")).expect("Fixtures:");
        let upload_id = "041bae61-adb4-4ba2-80eb-48719396f0e3";
        let path = format!("/{}", upload_id);
        let _m = mock("GET", path.as_str())
          .with_status(200)
          .match_header("Authorization", "Bearer jwt-token")
          .match_header("x-api-key", "1234")
          .with_body(body)
          .create();
        
        let service = TransferService::new("jwt-token", "1234");
        let transfer = service.find(upload_id).unwrap();
        assert_eq!(transfer.id, upload_id);
        assert_eq!(transfer.url.unwrap(), "https://we.tl/t-12344657");
        assert_eq!(transfer.state, "processing");
    }
}
