use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use rand::{rngs::OsRng, RngCore};
use std::fs::{self, File};
use std::io::{Write, Error};
use std::env;

type Aes256CbcEncryption = Cbc<Aes256, Pkcs7>;

fn generate_encryption_key_and_iv() -> (Vec<u8>, Vec<u8>) {
    let mut encryption_key = vec![0u8; 32];
    let mut initialization_vector = vec![0u8; 16];
    OsRng.fill_bytes(&mut encryption_key);
    OsRng.fill_bytes(&mut initialization_vector);
    (encryption_key, initialization_vector)
}

pub fn encrypt_file_at_path(file_path: &str, encrypted_file_path: &str) -> Result<(), Error> {
    let file_contents = fs::read(file_path)?;
    let (encryption_key, initialization_vector) = generate_encryption_key_and_iv();

    let cipher = match Aes256CbcEncryption::new_from_slices(&encryption_key, &initialization_vector) {
        Ok(c) => c,
        Err(e) => return Err(Error::new(std::io::ErrorKind::Other, e)),
    };

    let encrypted_data = cipher.encrypt_vec(&file_contents);
    
    let mut encrypted_file = File::create(encrypted_file_path)?;
    encrypted_file.write_all(&initialization_vector)?;
    encrypted_file.write_all(&encryption_key)?; 
    encrypted_file.write_all(&encrypted_data)?;
    Ok(())
}

pub fn decrypt_file_at_path(encrypted_file_path: &str, decrypted_file_path: &str) -> Result<(), Error> {
    let encrypted_file_contents = fs::read(encrypted_file_path)?;
    if encrypted_file_contents.len() < 48 { 
        return Err(Error::new(std::io::ErrorKind::InvalidInput, "Encrypted file is too short."));
    }
    let (initialization_vector, rest) = encrypted_file_contents.split_at(16);
    let (encryption_key, encrypted_data) = rest.split_at(32);

    let cipher = match Aes256CbcEncryption::new_from_slices(encryption_key, initialization_vector) {
        Ok(c) => c,
        Err(e) => return Err(Error::new(std::io::ErrorKind::Other, e)),
    };

    let decrypted_data = cipher.decrypt_vec(encrypted_data)?;
    fs::write(decrypted_file_path, decrypted_data)?;
    Ok(())
}