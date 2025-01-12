use crate::requests::Client;

mod requests;
mod encryption;
mod response;
mod tests;
mod api_keys;



fn main() {
    // let client = Client::from_toml_file(String::from("config.toml"));
    // dbg!(client.clone());
    //
    // println!("{:?}", client.get_all_passwords().await);
    tests::mini_stress_test();
}
