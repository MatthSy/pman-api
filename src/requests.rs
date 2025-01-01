use std::fs;
use serde::{Deserialize, Serialize};
use crate::encryption::EncryptedData;
use crate::response::ApiResponse;

use toml;

#[allow(unused)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Client {
    url: String,
    password_key: Option<String>,
    api_key: Option<String>,
}

#[allow(unused)]
impl Client {

    pub fn url(&self) -> String { self.url.clone() }
    pub fn password_key(&self) -> Option<String> { self.password_key.clone() }
    pub fn api_key(&self) -> Option<String> { self.api_key.clone() }

    pub fn new() -> Self {
        Client {
            url: "http://localhost:8000".to_string(),
            password_key: None,
            api_key: None,
        }
    }

    pub fn from_toml(toml: String) -> Self {
        // TODO
        Client::new()
    }

    pub fn from_toml_file(file: String) -> Self {
        let client: Self = toml::from_str(
            &fs::read_to_string(file).expect("Cannot load config file")
        ).unwrap();
        client
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
        let url = format!("{}/passwords/{}", self.url(), password_id);
        _get_passwords(self, url).await
    }

    #[allow(unused)]
    pub async fn get_all_passwords(&self) -> ApiResponse {
        let url =  format!("{}/passwords", self.url());
        _get_passwords(self, url).await
    }

    #[allow(unused)]
    pub async fn post_encrypted_password(&self, encrypted_data: EncryptedData) -> ApiResponse {
        let url = format!("{}/passwords/{}", self.url(), encrypted_data.id);

        let mut serialized_data = toml::to_string(&encrypted_data);
        if serialized_data.is_err() {
            let err_msg = serialized_data.err().unwrap().to_string();
            return ApiResponse::from(400, Some(format!("Error while serializing data : {}", err_msg)), None);
        }

        let client = reqwest::Client::new();
        let response = client.post(url)
            .body(serialized_data.unwrap())
            .header("X-API-KEY", self.api_key().unwrap_or(String::new()))
            .send()
            .await;

        _treat_response(response).await
    }

    #[allow(unused)]
    pub async fn post_reload_api_keys(&self) -> ApiResponse {
        let url = format!("{}/api_keys/reload", self.url());
        let client = reqwest::Client::new();
        let response = client.post(url)
            .header("X-API-KEY", self.api_key().unwrap_or(String::new()))
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
async fn _get_passwords(client: &Client, url: String) -> ApiResponse {
    let reqw_client = reqwest::Client::new();
    let response = reqw_client.get(url)
        .header("X-API-KEY", client.api_key().unwrap_or(String::new()))
        .send()
        .await;
    _treat_response(response).await
}