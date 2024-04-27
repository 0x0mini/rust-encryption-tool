use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use rand::{rngs::OsRng, RngCore};
use std::fs::{self, File};
use std::io::{Write, Error};
use std::env;

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

fn generate_key_iv() -> (Vec<u8>, Vec<u8>) {
    let mut key = vec![0u8; 32]; 
    let mut iv = vec![0u8; 16]; 
    OsRng.fill_bytes(&mut key);
    OsRng.fill_bytes(&mut iv);
    (key, iv)
}

pub fn encrypt_file(file_path: &str, output_path: &str) -> Result<(), Error> {
    let content = fs::read(file_path)?;
    let (key, iv) = generate_key_iv();
    let cipher = Aes256Cbc::new_from_slices(&key, &iv).unwrap();
    let encrypted_data = cipher.encrypt_vec(&content);
    
    let mut output = File::create(output_path)?;
    output.write_all(&iv)?; 
    output.write_all(&encrypted_data)?;
    Ok(())
}

pub fn decrypt_file(file_path: &str, output_path: &str, key: &[u8], iv: &[u8]) -> Result<(), Error> {
    let content = fs::read(file_path)?;
    if content.len() < 16 {
        return Err(Error::new(std::io::ErrorKind::InvalidInput, "File is too short to be valid"));
    }
    let cipher = Aes256Cbc::new_from_slices(key, iv).unwrap();
    let decrypted_data = cipher.decrypt_vec(&content[16..])?;
    fs::write(output_path, decrypted_data)?;
    Ok(())
}