use ring::error::Unspecified;
use ring::rand::{SecureRandom, SystemRandom};
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::num::ParseIntError;

const DEFAULT_KEY_LENGTH: usize = 32;
const ENCRYPTION_KEY_FILENAME: &str = "encryption_key";

#[derive(Debug)]
enum EncryptionError {
    Io(io::Error),
    CryptoError(Unspecified),
    Parse(ParseIntError),
    EnvVarError(env::VarError),
    General(String), // General error for more customized error messages
}

impl From<io::Error> for EncryptionError {
    fn from(error: io::Error) -> Self {
        EncryptionError::Io(error)
    }
}

impl From<Unspecified> for EncryptionError {
    fn from(error: Unspecified) -> Self {
        EncryptionError::CryptoError(error)
    }
}

impl From<ParseIntError> for EncryptionError {
    fn from(error: ParseIntError) -> Self {
        EncryptionError::Parse(error)
    }
}

impl From<env::VarError> for EncryptionError {
    fn from(error: env::VarError) -> Self {
        EncryptionError::EnvVarError(error)
    }
}

pub fn generate_encryption_key() -> Result<Vec<u8>, EncryptionError> {
    let key_length_env = env::var("KEY_LENGTH")
        .or_else(|err| {
            eprintln!("Failed to read KEY_LENGTH environment variable: {:?}", err);
            Err(err)
        })
        .unwrap_or_else(|_| DEFAULT_KEY_LENGTH.to_string())
        .parse::<usize>()?;

    let rng = SystemRandom::new();
    let mut key = vec![0u8; key_length_env];
    rng.fill(&mut key).map_err(|err| {
        eprintln!("Failed to fill encryption key with random bytes: {:?}", err);
        EncryptionError::from(err)
    })?;

    Ok(key)
}

pub fn store_encryption_key(key: &[u8]) -> Result<(), EncryptionError> {
    let key_storage_path = env::var("KEY_PATH")
        .or_else(|err| {
            eprintln!("Failed to read KEY_PATH environment variable: {:?}", err);
            Err(err)
        })
        .unwrap_or_else(|_| String::from("."));
    let full_key_file_path = Path::new(&key_storage_path).join(ENCRYPTION_KEY_FILENAME);

    let mut file = File::create(&full_key_file_path).map_err(|err| {
        eprintln!("Failed to create file for storing encryption key: {:?}", err);
        EncryptionError::from(err)
    })?;
    file.write_all(key).map_err(|err| {
        eprintln!("Failed to write encryption key to file: {:?}", err);
        EncryptionError::from(err)
    })?;

    println!("Encryption key saved to: {}", full_key_file_path.display());

    Ok(())
}

pub fn retrieve_encryption_key() -> Result<Vec<u8>, EncryptionError> {
    let key_storage_path = env::var("KEY_PATH")
        .or_else(|err| {
            eprintln!("Failed to read KEY_PATH environment variable: {:?}", err);
            Err(err)
        })
        .unwrap_or_else(|_| String::from("."));
    let full_key_file_path = Path::new(&key_storage_path).join(ENCRYPTION_KEY_FILENAME);

    let mut file = File::open(&full_key_file_path).map_err(|err| {
        eprintln!("Failed to open encryption key file: {:?}", err);
        EncryptionError::from(err)
    })?;
    let mut key = Vec::new();
    file.read_to_end(&mut key).map_err(|err| {
        eprintln!("Failed to read encryption key from file: {:?}", err);
        EncryptionError::from(err)
    })?;

    Ok(key)
}

fn main() -> Result<(), EncryptionError> {
    match generate_encryption_key() {
        Ok(key) => {
            store_encryption_key(&key)?;
            println!("Encryption key generation and storage successful.");
        }
        Err(error) => {
            println!("Failed to generate encryption key: {:?}", error);
            return Err(error);
        }
    }

    match retrieve_encryption_key() {
        Ok(key) => println!("Encryption key retrieved. Length: {}", key.len()),
        Err(error) => {
            println!("Failed to retrieve encryption key: {:?}", error);
            return Err(error);
        }
    }

    Ok(())
}