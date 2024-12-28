mod requests;
mod encryption;
mod response;

use crate::encryption::{decrypt_password, decrypt_password_from_toml, encrypt_password};

#[tokio::main]
async fn main() {
    let mut binding = requests::Client::new();
    let mut client = binding.set_url(String::from("http://127.0.0.1:8000"));
    let encrypted = encrypt_password(
        String::from("VeryHardToRememberPassword"),
        String::from("pass1"),
        String::from("AZERTY")
    ).expect("couldnt encrypt");

    dbg!(&encrypted);
    let post_response = dbg!(client.post_password(encrypted).await);
    if post_response.is_err() { return }

    let get_response = dbg!(client.get_password("pass1").await);
    println!("Decrypted : {:?}", decrypt_password_from_toml(get_response.msg().expect("Could decrypt"), String::from("AZERTY")));
}
