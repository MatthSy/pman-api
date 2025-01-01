use sha2::{Sha256, Digest};
use ring::aead::{Aad, AES_256_GCM, BoundKey, Nonce, NONCE_LEN, NonceSequence, OpeningKey, SealingKey, UnboundKey};
use ring::error::Unspecified;
use serde::{Serialize, Deserialize};

#[allow(unused)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EncryptedData {
    pub(crate) password: [u8; 32],
    pub(crate) id: String,
    pub(crate) tag: [u8; 16],
}

#[allow(unused)]
#[derive(Debug, Copy, Clone)]
pub enum EncryptionError {
    UnboundKeyErr,
    SealingErr,
    TagParsingErr,
    IncorrectPassword,
    ParsingErr,
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
pub fn encrypt_password(input: String, password_id: String, password_key: String) -> Result<EncryptedData, EncryptionError> {
    let hashed_key = hash_password(password_key);

    let unbound_key = UnboundKey::new(&AES_256_GCM, &hashed_key);
    if unbound_key.is_err() { return Err(EncryptionError::UnboundKeyErr) }
    let unbound_key = unbound_key.unwrap();

    let nonce_sequence = CounterNonceSequence(0);

    let associated_data = Aad::from(&password_id);
    let mut in_out = [0; 32];
    let input_bytes = input.as_bytes();
    for i in 0..input.len() {
        in_out[i] = input_bytes[i];
    }

    // Encryption
    let mut sealing_key = SealingKey::new(unbound_key, nonce_sequence);
    let tag = sealing_key.seal_in_place_separate_tag(associated_data, &mut in_out);
    if tag.is_err() { return Err(EncryptionError::SealingErr)}

    // Parsing the tag as [u8; 16]
    let tag: Result<[u8; 16], _> = tag.unwrap().as_ref().try_into();
    if tag.is_err() { return Err(EncryptionError::TagParsingErr) }
    let tag = tag.unwrap();

    Ok(EncryptedData {
        password: <[u8; 32]>::try_from(in_out).unwrap(),
        id: password_id,
        tag,
    })
}

#[allow(unused)]
pub fn decrypt_password(encrypted_data: EncryptedData, password_key: String) -> Result<String, EncryptionError> {
    let hashed_key = hash_password(password_key);

    let unbound_key = UnboundKey::new(&AES_256_GCM, &hashed_key);
    if unbound_key.is_err() { return Err(EncryptionError::UnboundKeyErr) }
    let unbound_key = unbound_key.unwrap();

    let nonce_sequence = CounterNonceSequence(0);

    let associated_data = Aad::from(&encrypted_data.id);

    let mut opening_key = OpeningKey::new(unbound_key, nonce_sequence);

    let mut cypher_text_with_tag = [&encrypted_data.password, encrypted_data.tag.as_ref()].concat();
    let associated_data = Aad::from(&encrypted_data.id);

    let decrypted_data = opening_key.open_in_place(associated_data, &mut cypher_text_with_tag);
    if decrypted_data.is_err() { return Err(EncryptionError::IncorrectPassword)}

    // Remove the trailing 0s
    let decrypted_data= decrypted_data.unwrap().split(|c| *c == 0).next().unwrap();

    Ok(String::from_utf8(decrypted_data.to_vec()).unwrap())
}


#[allow(unused)]
pub fn decrypt_password_from_toml(serialized_data: String, password_key: String) -> Result<String, EncryptionError> {
    let encrypted_data: Result<EncryptedData, _> = toml::from_str(&*serialized_data);
    if encrypted_data.is_err() { return Err(EncryptionError::ParsingErr) }
    let encrypted_data = encrypted_data.unwrap();

    decrypt_password(encrypted_data, password_key)
}