use crate::requests::Client;

mod requests;
mod encryption;
mod response;
mod tests;
mod api_keys;

#[tokio::main]
async fn main() {
    let client = Client::from_toml_file(String::from("config.toml"));
    dbg!(client.clone());

    println!("{:?}", client.post_reload_api_keys().await);
}
