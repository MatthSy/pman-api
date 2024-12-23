mod requests;

use reqwest;
use crate::requests::get_password;

#[tokio::main]
async fn main() {
    let response = get_password("pass1").await.unwrap();
    println!("Response: {}", response);
}
