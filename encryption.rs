use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use rand::{rngs::OsRng, RngCore};
use std::{fs::{self, File}, io::{Write, Error, ErrorKind}};
use clap::{App, Arg, SubCommand};
use thiserror::Error;

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

#[derive(Debug, Error)]
enum MyError {
    #[error("Error with the file system: {0}")]
    FileSystem(#[from] std::io::Error),
    #[error("Error with crypto operation: {0}")]
    Crypto(String),
    #[error("The encrypted file is too short.")]
    ShortFile,
}

fn generate_key_and_iv() -> (Vec<u8>, Vec<u8>) {
    let mut key = vec![0u8; 32];
    let mut iv = vec![0u8; 16];
    OsRng.fill_bytes(&mut key);
    OsRng.fill_bytes(&mut iv);
    (key, iv)
}

pub fn encrypt_file(input_file_path: &str, output_encrypted_file_path: &str) -> Result<(), MyError> {
    let plain_text = fs::read(input_file_path)?;
    let (key, iv) = generate_key_and_iv();
    let cipher = Aes256Cbc::new_from_slices(&key, &iv)
        .map_err(|e| MyError::Crypto(e.to_string()))?;
    let encrypted_text = cipher.encrypt_vec(&plain_text);

    let mut encrypted_file = File::create(output_encrypted_file_path)?;
    encrypted_file.write_all(&iv)?;
    encrypted_file.write_all(&key)?;
    encrypted_file.write_all(&encrypted_text)?;
    Ok(())
}

pub fn decrypt_file(input_encrypted_file_path: &str, output_decrypted_file_path: &str) -> Result<(), MyError> {
    let encrypted_contents = fs::read(input_encrypted_file_path)?;
    if encrypted_contents.len() < 48 {
        return Err(MyError::ShortFile);
    }
    let (iv, remaining) = encrypted_contents.split_at(16);
    let (key, encrypted_text) = remaining.split_at(32);

    let cipher = Aes256Cbc::new_from_slices(key, iv)
        .map_err(|e| MyError::Crypto(e.to_string()))?;
    let decrypted_text = cipher.decrypt_vec(encrypted_text)?;
    fs::write(output_decrypted_file_path, decrypted_text)?;
    Ok(())
}

fn main() {
    let matches = App::new("Rust Encryption Tool")
        .about("Encrypts or decrypts files")
        .subcommand(SubCommand::with_name("encrypt")
            .about("Encrypts a file")
            .arg(Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1))
            .arg(Arg::with_name("OUTPUT")
                .help("Sets the output file for the encrypted data")
                .required(true)
                .index(2)))
        .subcommand(SubCommand::with_name("decrypt")
            .about("Decrypts a file")
            .arg(Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1))
            .arg(Arg::with_name("OUTPUT")
                .help("Sets the output file for the decrypted data")
                .required(true)
                .index(2)))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("encrypt") {
        let input_file = matches.value_of("INPUT").unwrap();
        let output_file = matches.value_of("OUTPUT").unwrap();
        if let Err(e) = encrypt_file(input_file, output_file) {
            eprintln!("Error encrypting file: {}", e);
        }
    } else if let Some(matches) = matches.subcommand_matches("decrypt") {
        let input_file = matches.value_of("INPUT").unwrap();
        let output_file = matches.value_of("OUTPUT").unwrap();
        if let Err(e) = decrypt_file(input_file, output_file) {
            eprintln!("Error decrypting file: {}", e);
        }
    }
}