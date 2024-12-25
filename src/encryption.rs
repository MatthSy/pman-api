use sha2::{Sha256, Digest};
use ring::aead::{Aad, AES_256_GCM, BoundKey, Nonce, NONCE_LEN, NonceSequence, OpeningKey, SealingKey, UnboundKey};
use ring::error::Unspecified;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EncryptedData {
    pub(crate) password: [u8; 32],
    pub(crate) id: String,
    pub(crate) tag: [u8; 16],
}

#[allow(unused)]
pub fn hash_password(str: String) -> [u8; 32]{
    Sha256::digest(&str).into()
}

#[allow(unused)]
pub struct CounterNonceSequence(pub u32);

#[allow(unused)]
impl NonceSequence for CounterNonceSequence {
    // called once for each seal operation
    fn advance(&mut self) -> Result<Nonce, Unspecified> {
        let mut nonce_bytes = vec![0; NONCE_LEN];

        let bytes = self.0.to_be_bytes();
        nonce_bytes[8..].copy_from_slice(&bytes);
        // println!("nonce_bytes = {}", hex::encode(&nonce_bytes));

        self.0 += 1; // advance the counter
        Nonce::try_assume_unique_for_key(&nonce_bytes)
    }
}

#[allow(unused)]
// TODO : change return type to an Err enum
pub fn encrypt_password(input: String, password_id: String, password_key: String) -> EncryptedData {
    let hashed_key = hash_password(password_key);

    let unbound_key = UnboundKey::new(&AES_256_GCM, &hashed_key).expect("Unbound key creation fail");
    let nonce_sequence = CounterNonceSequence(0);

    let associated_data = Aad::from(&password_id);
    let mut in_out = [0; 32];
    let input_bytes = input.as_bytes();
    for i in 0..input.len() {
        in_out[i] = input_bytes[i];
    }

    let mut sealing_key = SealingKey::new(unbound_key, nonce_sequence);
    let tag = sealing_key.seal_in_place_separate_tag(associated_data, &mut in_out).expect("Encrypting error");

    EncryptedData {
        password: <[u8; 32]>::try_from(in_out).unwrap(),
        id: password_id,
        tag: tag.as_ref().try_into().expect("Error parsing the tag in encrypt_password function"),
    }
}

#[allow(unused)]
// TODO : change return type to an Err enum
pub fn decrypt_password(encrypted_data: EncryptedData, password_key: String) -> String {
    let hashed_key = hash_password(password_key);

    let unbound_key = UnboundKey::new(&AES_256_GCM, &hashed_key).expect("Unbound key creation fail");
    let nonce_sequence = CounterNonceSequence(0);

    let associated_data = Aad::from(&encrypted_data.id);
    let mut in_out = [0; 32];

    let mut opening_key = OpeningKey::new(unbound_key, nonce_sequence);

    let mut cypher_text_with_tag = [&encrypted_data.password, encrypted_data.tag.as_ref()].concat();
    let associated_data = Aad::from(&encrypted_data.id);

    let decrypted_data = opening_key.open_in_place(associated_data, &mut cypher_text_with_tag).expect("Error, incorrect password or unknown internal error");

    String::from_utf8(decrypted_data.to_vec()).unwrap()
}