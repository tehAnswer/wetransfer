use serde::de::DeserializeOwned;
use serde::Serialize;
use reqwest::{Response, Client};
use reqwest::header::{AUTHORIZATION, ACCEPT, CONTENT_TYPE, HeaderValue, HeaderMap, HeaderName};
use responses::WeTransferError;
use std::error::Error;

/// A wrapper around `reqwest::Client` that builds requests 
/// and parses their responses into well-known structs.
#[derive(Debug)]
pub struct RequestService {
    http_client: Client,
    jwt: String,
    app_token: String,
    base_url: String
}

impl RequestService {
    /// Constructor.
    pub fn new<S: Into<String>+ToString>(jwt: S, app_token: S, base_url: String) -> RequestService {
        RequestService {
            http_client: Client::new(),
            jwt: jwt.to_string(),
            app_token: app_token.to_string(),
            base_url: base_url.to_string()
        }
    }

    /// Performs a GET HTTP requests
    pub fn get<U: DeserializeOwned>(&self, path: &str) -> Result<U, WeTransferError> {
        let url = format!("{}{}", self.base_url, path);
        let result = self.http_client
            .get(url.as_str())
            .headers(self.construct_headers())
            .send();
        self.handle_response(result)
    }

    /// Performs a POST HTTP request
    pub fn post<T: Serialize, U: DeserializeOwned>(&self, path: &str, payload: T) -> Result<U, WeTransferError> {
        let url = format!("{}{}", self.base_url, path);
        let result = self.http_client
            .post(url.as_str())
            .headers(self.construct_headers())
            .json(&payload).send();
        self.handle_response(result)
    }

    /// Performs a PUT HTTP request
    pub fn put<T: Serialize, U: DeserializeOwned>(&self, path: &str, payload: T) -> Result<U, WeTransferError> {
        let url = format!("{}{}", self.base_url, path);
        let result = self.http_client
            .put(url.as_str())
            .headers(self.construct_headers())
            .json(&payload).send();
        self.handle_response(result)
    }

    /// Performs a file upload using a presigned S3 url.
    pub fn file_upload<S: Into<String>+ToString>(&self, url: S, io: &[u8]) -> Result<reqwest::Response, WeTransferError> {
        let result = self.http_client
            .put(url.to_string().as_str())
            .body(io.to_vec())
            .send();
        match result {
            Ok(response) => {
                if response.status().is_success() {
                    Ok(response) 
                } else {
                    Err(WeTransferError{
                        status: response.status().as_u16(),
                        message: String::from("S3 file upload failed"),
                    })
                }
            },
            Err(error_raw) => Err(WeTransferError {
                status: 0,
                message: format!("Error while uploading file for transfer: {}", error_raw.description().to_string())
            })
        }
    }

    fn handle_response<U: DeserializeOwned>(&self, result: Result<Response, reqwest::Error>) -> Result<U, WeTransferError> {
        match result {
            Ok(mut response) => {
                if response.status().is_success() {
                    match response.json::<U>() {
                        Ok(final_response) => Ok(final_response),
                        Err(error) => {
                            Err(WeTransferError { status: 0, message: error.to_string() })
                        }
                    }
                } else {
                    let parsed_result = response.json::<WeTransferError>();
                    println!("{:?}", parsed_result);
                    match parsed_result {
                        Ok(mut wetransfer_error) => {
                            wetransfer_error.status = response.status().as_u16();
                            Err(wetransfer_error)
                        },
                        Err(_) => Err(WeTransferError{
                            status: 0,
                            message: String::from("Error while parsing a WeTransfer error response. Contact mainteners.")
                        }) 
                    }
                }
            },
            Err(error) => Err(WeTransferError {
                status: 0,
                message: error.description().to_string()
            })
        }
    }

    fn construct_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let api_key_value= HeaderValue::from_str(self.app_token.as_str()).unwrap();
        let jwt_value = HeaderValue::from_str(format!("Bearer {}", self.jwt).as_str()).unwrap();
        headers.insert(HeaderName::from_static("x-api-key"), api_key_value);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(AUTHORIZATION, jwt_value);
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;

    #[test]
    fn it_uploads_files_to_s3() {
        let url = format!("{}/upload", mockito::SERVER_URL);
        let _m = mock("PUT", "/upload").with_status(200).create();
        let service = RequestService::new("jwt-token", "1234", mockito::SERVER_URL.to_string());
        let result = service.file_upload(url, &[0;10]);
        assert!(result.is_ok());
    }
}
