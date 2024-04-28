use ring::rand::{SecureRandom, SystemRandom};
use ring::error::Unspecified;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::env;

const KEY_LENGTH: usize = 32;
const KEY_FILE: &str = "encryption_key";

pub fn generate_key() -> Result<[u8; KEY_LENGTH], Unspecified> {
    let rng = SystemRandom::new();
    let mut key: [u8; KEY_LENGTH] = [0; KEY_LENGTH];
    rng.fill(&mut key)?;
    Ok(key)
}

pub fn save_key(key: &[u8]) -> Result<(), std::io::Error> {
    let key_path = env::var("KEY_PATH")
        .unwrap_or_else(|_| String::from("."));
    let key_file_path = Path::new(&key_path).join(KEY_FILE);

    let mut file = File::create(key_file_path)?;
    file.write_all(key)?;
    Ok(())
}

pub fn retrieve_key() -> Result<Vec<u8>, std::io::Error> {
    let key_path = env::var("KEY_PATH")
        .unwrap_or_else(|_| String::from("."));
    let key_file_path = Path::new(&key_path).join(KEY_FILE);

    let mut file = File::open(key_file_path)?;
    let mut key = Vec::new();
    file.read_to_end(&mut key)?;
    Ok(key)
}