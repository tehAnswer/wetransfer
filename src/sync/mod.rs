extern crate reqwest;
pub mod transfer;
pub mod board;

use responses::Login;


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
        http_client
          .post("https://dev.wetransfer.com/v2/authorize")
          .header("x-api-key", app_token.to_string())
          .send().unwrap()
          .json::<Login>().unwrap()
    }

}
