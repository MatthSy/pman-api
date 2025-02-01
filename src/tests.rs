use crate::requests::Client;

#[allow(unused)]
pub fn mini_stress_test() {
    let client = Client::from_toml_file(String::from("config.toml"));
    let rt = tokio::runtime::Runtime::new().unwrap();

    for _ in 0..100 {
        rt.block_on(async {
            let res = client.get_all_passwords().await;
            println!("{:?}", res);
        });
        rt.block_on(async {
            let res = client.get_index().await;
            println!("{:?}", res);
        });
        rt.block_on(async {
            let res = client.get_password("Test").await;
            println!("{:?}", res);
        });
        rt.block_on(async {
            let res = client.get_password("nop").await;
            println!("{:?}", res);
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::encryption::{decrypt_password, decrypt_password_from_toml, encrypt_password};
    use crate::requests::Client;

    #[tokio::test]
    async fn test_encrypt_decrypt() {
        let encrypted = encrypt_password(
            String::from("FooFightersFan"),
            String::from("Test1"),
            String::from("123456")
        ).expect("couldnt encrypt");

        let decrypted = decrypt_password(encrypted, String::from("123456")).expect("couldnt decrypt");

        assert_eq!(decrypted, "FooFightersFan");
    }

    #[tokio::test]
    async fn test_encrypt_decrypt_incorrect_password_key() {
        let encrypted = encrypt_password(
            String::from("CreedEnjoyer"),
            String::from("Test2"),
            String::from("HoldMeNowImSixFeetFromTheEdge")
        ).expect("couldnt encrypt");

        decrypt_password(
            encrypted,
            String::from("I dont like creed (illegal)")
        ).expect_err("couldnt decrypt");
    }

    #[tokio::test]
    async fn test_post_get_decrypt() {
        let client = Client::from_toml_file(String::from("config.toml"));

        dbg!(client.clone());

        let encrypted = encrypt_password(
            String::from("VeryHardToRememberPassword"),
            String::from("Test3"),
            String::from("AZERTY")
        ).expect("couldnt encrypt");

        let post_response = client.post_encrypted_password(encrypted).await;
        if post_response.is_err() { panic!() }

        let get_response = client.get_password("Test3").await;
        let decrypted =
            decrypt_password_from_toml(
                get_response.msg().expect("No msg"),
                String::from("AZERTY")
            ).expect("Could not decrypt");

        assert_eq!(decrypted, "VeryHardToRememberPassword");
    }

    #[tokio::test]
    async fn test_post_get_decrypt_incorrect_password_key() {
        let client = Client::from_toml_file(String::from("config.toml"));

        let encrypted = encrypt_password(
            String::from("VeryHardToRememberPassword"), // Password to save
            String::from("Test4"), // Password ID
            String::from("AZERTY") // Password key, to encrypt and decrypt
        ).expect("couldnt encrypt");

        let post_response = client.post_encrypted_password(encrypted).await;
        if post_response.is_err() { panic!() }

        let get_response = client.get_password("Test4").await;
        if get_response.is_err() { panic!() }

        decrypt_password_from_toml(
            get_response.msg().expect("No msg"),
            String::from("azerty")
        ).expect_err("");
        dbg!(get_response);
    }
}