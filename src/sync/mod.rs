use responses::Login;
use responses::WeTransferError;

pub mod transfer;
pub mod board;
pub mod requester;

#[cfg(not(test))]
const LOGIN_URL: &'static str = "https://dev.wetransfer.com/v2/authorize";
#[cfg(test)]
const LOGIN_URL: &'static str = mockito::SERVER_URL;

#[derive(Debug)]
pub struct Client {
    pub transfers: transfer::TransferService,
    pub boards: board::BoardService,
}

impl Client {
    pub fn new<S: Into<String>+ToString>(app_token: S) -> Result<Client, WeTransferError> {
        let result = Client::login(app_token.to_string());
        match result {
            Ok(login) => {
                let jwt = login.token;
                let client = Client {
                    transfers: transfer::TransferService::new(jwt.clone(), app_token.to_string()),
                    boards: board::BoardService::new(jwt.clone(), app_token.to_string())
                };
                Ok(client)
            },
            Err(error) => Err(error),
        }
    }

    fn login<S: Into<String>+ToString>(app_token: S) -> Result<Login, WeTransferError> {
        let http_client = reqwest::Client::new();
        let mut response = http_client
          .post(LOGIN_URL)
          .header("x-api-key", app_token.to_string())
          .send().unwrap();
        if response.status().is_success() {
            Ok(response.json::<Login>().unwrap())
        } else {
            let mut error_response = response.json::<WeTransferError>().unwrap();
            error_response.status = response.status().as_u16();
            Err(error_response)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;

    #[test]
    fn it_logins() {
        let app_token = "1234";
        let _m = mock("POST", "/")
          .with_status(200)
          .match_header("x-api-key", app_token)
          .with_body("{\"token\": \"jwt_token\", \"success\": true}")
          .create();

        let client_creation = Client::new(app_token);
        assert!(client_creation.is_ok());
    }

    #[test]
    fn it_returns_error() {
        let app_token = "1234";
        let _m = mock("POST", "/")
          .with_status(401)
          .match_header("x-api-key", app_token)
          .with_body("{\"message\": \"You suck.\", \"success\": false}")
          .create();

        let error = Client::new(app_token).unwrap_err();
        assert_eq!(error.message, "You suck.");
        assert_eq!(error.status, 401);
    }
}
