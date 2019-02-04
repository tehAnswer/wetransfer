use std::path::Path;
use sync::requester::RequestService;
use responses::*;
use requests::*;
use std::fs;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;

#[derive(Debug)]
pub struct BoardService {
    requester: RequestService
}

#[cfg(not(test))]
const BOARDS_URL: &'static str = "https://dev.wetransfer.com/v2/boards";
#[cfg(test)]
const BOARDS_URL: &'static str = mockito::SERVER_URL;

impl BoardService {
    pub fn new<S: Into<String>+ToString>(jwt: S, app_token: S) -> BoardService {
        BoardService {
            requester: RequestService::new(jwt.to_string(), app_token.to_string(), BOARDS_URL.to_owned()),
        }
    }

    pub fn create<S: Into<String>+ToString>(&self, name: S, description: Option<String>) -> Result<Board, WeTransferError> {
        let payload = CreateBoardRequest { name: name.to_string(), description: description };
        self.requester.post::<CreateBoardRequest, Board>("/", payload)
    }
    
    pub fn add_links<S: Into<String>+ToString>(&self, board_id: S, links: Vec<AddLink>) -> Result<Vec<Link>, WeTransferError> {
        let path = format!("/{}/links", board_id.to_string());
        self.requester.post::<Vec<AddLink>, Vec<Link>>(&path, links)
    }

    pub fn add_files(&self, board_id: String, paths: &Vec<String>) -> Result<(), WeTransferError> {
        let result = self.start_file_uploads(board_id.to_string(), paths);
        match result {
            Ok(list_of_files) => self.fulfill_file_uploads(board_id.to_string(), &list_of_files, paths),
            Err(wetransfer_error) => Err(wetransfer_error)
        }
    }

    fn start_file_uploads<S: Into<String>+ToString>(&self, board_id: S, paths: &Vec<S>) -> Result<Vec<FileBoard>, WeTransferError> {
        let files = paths.into_iter().map(|path_str: &S| {
            // Compiler suggested to put this expression under
            // a let variable.
            let path_str_ref = path_str.to_string();
            let path = Path::new(&path_str_ref);
            self.extract_file_info(&path)
        }).collect();

        match files {
            Ok(file_requests) => {
                let path = format!("/{}/files", board_id.to_string());
                self.requester.post::<Vec<FileRequest>, Vec<FileBoard>>(&path, file_requests)
            },
            Err(transfer_error) => Err(transfer_error)
        }
    }

    fn fulfill_file_uploads<S: Into<String>+ToString>(&self, board_id: S, list_of_files: &Vec<FileBoard>, paths: &Vec<S>) -> Result<(), WeTransferError> {
        for (index, file) in list_of_files.iter().enumerate() {
            let path_str = paths.get(index).unwrap().to_string();
            let path = Path::new(path_str.as_str());
            let mut file_io = File::open(path).unwrap();
            let mut buffer = std::vec::from_elem(0, file.multipart.chunk_size as usize);
            for part in 1..=file.multipart.part_numbers {
                file_io.read(&mut buffer).unwrap();
                let s3_url = self.upload_url_for(board_id.to_string(), file.id.to_owned(), part, file.multipart.id.to_string())?.url;
                self.requester.file_upload(s3_url, &buffer).unwrap();
            }
            self.mark_as_complete(board_id.to_string(), file.id.to_owned())?;
        }
        Ok(())
    }
    
    fn upload_url_for<S: Into<String>+ToString>(&self, board_id: S, file_id: S, part: u64, multipart_id: S) -> Result<GetUploadUrlResponse, WeTransferError> {
        let path = format!("/{}/files/{}/upload-url/{}/{}", board_id.to_string(), file_id.to_string(), part, multipart_id.to_string());
        self.requester.get::<GetUploadUrlResponse>(&path)
    }

    fn mark_as_complete<S: Into<String>+ToString>(&self, board_id: S, file_id: S) -> Result<CompleteFileBoardUploadResponse, WeTransferError> {
        let payload = CompleteFileBoardUploadRequest {};
        let path = format!("/{}/files/{}/upload-complete", board_id.to_string(), file_id.to_string());
        self.requester.put::<CompleteFileBoardUploadRequest, CompleteFileBoardUploadResponse>(&path, payload)
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
    fn it_creates_boards() {
        let body = fs::read_to_string(Path::new("src/support/create_board.json")).expect("Fixtures:");
        let _m = mock("POST", "/")
          .with_status(201)
          .match_header("Authorization", "Bearer jwt-token")
          .match_header("x-api-key", "1234")
          .with_body(body)
          .create();
        
        let instance = BoardService::new("jwt-token", "1234");
        let result = instance.create("xd", None);
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.id, "swnoauod92ugkkhbj20190126151445");
        assert_eq!(response.name, "xd");
        assert_eq!(response.description, None);
        assert_eq!(response.state, "downloadable");
    }

    #[test]
    fn it_add_links() {
        let body = fs::read_to_string(Path::new("src/support/add_links.json")).expect("Fixtures:");
        let _m = mock("POST", "/id-board/links")
          .with_status(200)
          .match_header("Authorization", "Bearer jwt-token")
          .match_header("x-api-key", "1234")
          .with_body(body)
          .create();

        let instance = BoardService::new("jwt-token", "1234");
        let mut links: Vec<AddLink> = Vec::new();
        links.push(AddLink { url: String::from("https://wetransfer.com"), title: String::from("WeTransfer")});
        let result = instance.add_links("id-board", links);
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response[0].id, String::from("random-hash"));
        assert_eq!(response[0].url, String::from("https://wetransfer.com/"));
        assert_eq!(response[0].kind, String::from("link"));
        assert_eq!(response[0].meta.title, String::from("WeTransfer"));
    }

    #[test]
    fn it_requests_s3_urls_for_uploading_parts() {
        let upload_id = "041bae61-adb4-4ba2-80eb-48719396f0e3";
        let file_id = "783b21e9-f8f1-46d5-894c-62954109d11d";
        let multipart_id = "multipart_id";
        let path = format!("/{}/files/{}/upload-url/{}/{}", upload_id, file_id, 1, multipart_id);
        let _m = mock("GET", path.as_str())
          .with_status(201)
          .match_header("Authorization", "Bearer jwt-token")
          .match_header("x-api-key", "1234")
          .with_body("{\"success\": true, \"url\":\"https://s3-wetransfer.com/uploadhere\"}")
          .create();

        let service = BoardService::new("jwt-token", "1234");
        let upload_url_request = service.upload_url_for(upload_id, file_id, 1, multipart_id).unwrap();

        assert!(upload_url_request.success);
        assert_eq!(upload_url_request.url, "https://s3-wetransfer.com/uploadhere");
    }


    #[test]
    fn it_completes_file_board_uploads() {
        let body = fs::read_to_string(Path::new("src/support/complete_file_board_upload.json")).expect("Fixtures:");
        let upload_id = "041bae61-adb4-4ba2-80eb-48719396f0e3";
        let file_id = "783b21e9-f8f1-46d5-894c-62954109d11d";
        let path = format!("/{}/files/{}/upload-complete", upload_id, file_id);
        let _m = mock("PUT", path.as_str())
          .with_status(200)
          .match_header("Authorization", "Bearer jwt-token")
          .match_header("x-api-key", "1234")
          .with_body(body)
          .create();

        let service = BoardService::new("jwt-token", "1234");
        let result = service.mark_as_complete(upload_id, file_id);
        assert!(result.is_ok());
    }
}