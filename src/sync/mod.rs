extern crate reqwest;
#[cfg(test)]
extern crate mockito;
use responses::Login;

pub mod transfer;
pub mod board;

#[cfg(not(test))]
const LOGIN_URL: &'static str = "https://dev.wetransfer.com/v2/authorize";
#[cfg(test)]
const LOGIN_URL: &'static str = mockito::SERVER_URL;

#[derive(Debug)]
pub struct Client {
    pub transfers: transfer::Transfer,
    pub boards: board::Board,
}

impl Client {
    pub fn new<S: Into<String>+ToString>(app_token: S) -> Client {
        let jwt = Client::login(app_token).token;
        Client {
            transfers: transfer::Transfer { jwt: jwt.clone() },
            boards: board::Board { jwt: jwt.clone() }
        }
    }

    fn login<S: Into<String>+ToString>(app_token: S) -> Login {
        let http_client = reqwest::Client::new();
        let mut response = http_client
          .post(LOGIN_URL)
          .header("x-api-key", app_token.to_string())
          .send().unwrap();
        response.json::<Login>().unwrap()
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use self::mockito::mock;

    #[test]
    fn it_logins() {
        let app_token = "1234";
        let _m = mock("POST", "/")
          .with_status(200)
          .match_header("x-api-key", app_token)
          .with_body("{\"token\": \"jwt_token\", \"success\": true}")
          .create();

        let client = Client::new(app_token);
        assert_eq!(client.boards.jwt, "jwt_token");
        assert_eq!(client.transfers.jwt, "jwt_token");
    }
}
