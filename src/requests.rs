use serde::{Deserialize, Serialize};
use crate::encryption::EncryptedData;
use crate::response::ApiResponse;

use toml;

#[allow(unused)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Client {
    url: String,
    password_key: Option<String>
    // TODO : add other options
}

impl Client {

    pub fn new() -> Self {
        Client {
            url: "http://localhost:8000".to_string(),
            password_key: None,
        }
    }

    pub fn from_toml(toml: String) -> Self {
        // TODO
        Client::new()
    }

    pub fn from_toml_file(file: String) -> Self {
        // TODO
        Client::new()
    }

    pub fn set_url(&mut self, url: String) -> &mut Self {
        self.url = url;
        self
    }

    pub fn set_password_key(&mut self, password_key: Option<String>) -> &mut Self {
        self.password_key = password_key;
        self
    }



    #[allow(unused)]
    pub async fn get_password(&self, password_id: &str) -> ApiResponse {
        let url = format!("{}/passwords/{}", self.url, password_id);
        _get_passwords(url).await
    }

    #[allow(unused)]
    pub async fn get_all_passwords(&self) -> ApiResponse {
        let url =  format!("{}/passwords", self.url);
        _get_passwords(url).await
    }

    #[allow(unused)]
    pub async fn post_password(&self, encrypted_data: EncryptedData) -> ApiResponse {
        let url = format!("{}/passwords/{}", self.url, encrypted_data.id);

        let mut serialized_data = toml::to_string(&encrypted_data);
        if serialized_data.is_err() {
            let err_msg = serialized_data.err().unwrap().to_string();
            return ApiResponse::from(400, Some(format!("Error while serializing data : {}", err_msg)), None);
        }

        let client = reqwest::Client::new();
        let response = client.post(url)
            .body(serialized_data.unwrap())
            .send()
            .await;

        _treat_response(response).await
    }
}




// Private function for treating responses from the server
#[allow(unused)]
async fn _treat_response(response: Result<reqwest::Response, reqwest::Error>) -> ApiResponse {
    if response.is_err() {
        return ApiResponse::from(400, Some(String::from("Unknown Reqwest/Api error")), response.err());
    }

    let response = response.unwrap().text().await;
    if response.is_err() {
        let error = response.err().unwrap();
        return ApiResponse::from(error.status().unwrap().as_u16(), Some(error.to_string()), None);
    }

    let response = response.unwrap();
    if response != String::from(""){
        return ApiResponse::from(200, Some(response), None)
    }
    ApiResponse::from(200, None, None)
}

#[allow(unused)]
async fn _get_passwords(url: String) -> ApiResponse {
    let response = reqwest::get(&url).await;
    _treat_response(response).await
}