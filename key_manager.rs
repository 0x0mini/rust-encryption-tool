use ring::error::Unspecified;
use ring::rand::{SecureRandom, SystemRandom};
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

const DEFAULT_KEY_LENGTH: usize = 32;
const KEY_FILE: &str = "encryption_key";

#[derive(Debug)]
enum MyError {
    IoError(io::Error),
    UnspecifiedCryptoError(Unspecified),
}

impl From<io::Error> for MyError {
    fn from(e: io::Error) -> Self {
        MyError::IoError(e)
    }
}

impl From<Unspecified> for MyError {
    fn from(e: Unspecified) -> Self {
        MyError::UnspecifiedCryptoError(e)
    }
}

pub fn generate_key() -> Result<Vec<u8>, MyError> {
    let key_length = env::var("KEY_LENGTH")
        .unwrap_or_else(|_| DEFAULT_KEY_LENGTH.to_string())
        .parse()
        .unwrap_or(DEFAULT_KEY_LENGTH);

    let rng = SystemRandom::new();
    let mut key = vec![0u8; key_length];
    rng.fill(&mut key).map_err(MyError::from)?;
    Ok(key)
}

pub fn save_key(key: &[u8]) -> Result<(), MyError> {
    let key_path = env::var("KEY_PATH").unwrap_or_else(|_| String::from("."));
    let key_file_path = Path::new(&key_path).join(KEY_FILE);

    let mut file = File::create(&key_file_path)?;
    file.write_all(key)?;

    println!("Key saved to: {}", key_file_path.display());

    Ok(())
}

pub fn retrieve_key() -> Result<Vec<u8>, MyError> {
    let key_path = env::var("KEY_PATH").unwrap_or_else(|_| String::from("."));
    let key_file_path = Path::new(&key_path).join(KEY_FILE);

    let mut file = File::open(&key_file_path)?;
    let mut key = Vec::new();
    file.read_to_end(&mut key)?;

    Ok(key)
}

fn main() -> Result<(), MyError> {
    match generate_key() {
        Ok(key) => {
            save_key(&key)?;
            println!("Key generation and save successful.");
        }
        Err(err) => {
            println!("Failed to generate key: {:?}", err);
            return Err(err);
        }
    }

    match retrieve_key() {
        Ok(key) => println!("Key retrieved. Length: {}", key.len()),
        Err(err) => {
            println!("Failed to retrieve key: {:?}", err);
            return Err(err);
        }
    }

    Ok(())
}