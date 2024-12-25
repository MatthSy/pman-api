mod requests;
mod encryption;

use crate::encryption::{decrypt_password, encrypt_password};
#[allow(unused_imports)]
use crate::requests::{get_all_passwords};

#[tokio::main]
async fn main() {
    let encrypted = encrypt_password(String::from("AZERTY"), String::from("Site2"), String::from("hello"));
    println!("{:x?}", &encrypted);
    println!("Decrypted : {}", decrypt_password(encrypted, String::from("hello")));
}
