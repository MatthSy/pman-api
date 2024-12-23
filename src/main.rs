mod requests;

use reqwest;
use crate::requests::{get_all_passwords, get_password};

#[tokio::main]
async fn main() {
    let response = get_all_passwords().await.unwrap();
    println!("Response: {}", response);
}
