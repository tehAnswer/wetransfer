use std::path::Path;
use sync::requester::RequestService;
use responses::*;
use requests::*;

#[derive(Debug)]
pub struct BoardService {
    requester: RequestService,
}

#[cfg(not(test))]
const BOARDS_URL: &'static str = "https://dev.wetransfer.com/v2/boards";
#[cfg(test)]
const BOARDS_URL: &'static str = mockito::SERVER_URL;

impl BoardService {
    pub fn new<S: Into<String>+ToString>(jwt: S, app_token: S) -> BoardService {
        BoardService {
            requester: RequestService::new(jwt, app_token, BOARDS_URL.to_owned())
        }
    }

    pub fn create<S: Into<String>+ToString>(&self, name: S, description: Option<S>) -> Result<CreateBoardResponse, WeTransferError> {
        let payload = match description {
            Some(x) => CreateBoardRequest { name: name.to_string(), description: x.to_string() },
            None => CreateBoardRequest { name: name.to_string(), description: String::from("") }
        };
        self.requester.post::<CreateBoardRequest, CreateBoardResponse>("/", payload)
    }
    
    // pub fn add_link<S: Into<String>+ToString>(&self, name: S) -> Result<CreateBoardResponse, WeTransferError> {
        
    // }

    // pub fn add_file<S: Into<String>+ToString>(&self, name: S) -> Result<CreateBoardResponse, WeTransferError> {
        
    // }
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
        println!("{:?}", result);
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.id, "swnoauod92ugkkhbj20190126151445");
        assert_eq!(response.name, "xd");
        assert_eq!(response.description, None);
        assert_eq!(response.state, "downloadable");
    }

    #[test]
    fn it_add_links() {
    }

    #[test]
    fn it_add_files() {
    }

}