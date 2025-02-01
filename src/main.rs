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

    println!("{:?}", client.delete_password("Test4").await);
}

// fn main() {
//     tests::mini_stress_test();
// }
