use clap::{App, Arg};
use std::fs;
use std::io::{self, Error, ErrorKind};
use std::path::PathBuf;

mod encryption;

fn main() {
    let matches = build_cli().get_matches();

    let operation = matches.value_of("action").expect("Action argument is required");
    let source_file_path = matches.value_of("source_path").expect("Source file path argument is required");
    let mut destination_file_path = matches.value_of("destination_path").expect("Destination file path argument is required").to_string();
    let cipher_key = matches.value_of("encryption_key").expect("Encryption key argument is required").as_bytes();

    destination_file_path = change_file_extension_based_on_operation(operation, &destination_file_path);

    if let Err(error) = process_file(operation, source_file_path, &destination_file_path, cipher_key) {
        eprintln!("Error occurred while processing the file: {}", error);
    }
}

fn build_cli() -> App<'static> {
    App::new("File Encryption Tool")
        .version("1.0")
        .author("Your Name. <your_email@example.com>")
        .about("Encrypts or decrypts files")
        .arg(
            Arg::new("action")
                .help("Define whether to encrypt or decrypt")
                .required(true)
                .possible_values(&["encrypt", "decrypt"])
                .index(1),
        )
        .arg(
            Arg::new("source_path")
                .help("Path of the source file")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::new("destination_path")
                .help("Path for the output file")
                .required(true)
                .index(3),
        )
        .arg(
            Arg::new("encryption_key")
                .help("The key used for encryption or decryption")
                .required(true)
                .index(4),
        )
}

fn process_file(operation: &str, source_path: &str, destination_path: &str, key: &[u8]) -> io::Result<()> {
    match operation {
        "encrypt" => encrypt_or_decrypt_file(source_path, destination_path, key, "Encryption successful."),
        "decrypt" => encrypt_or_decrypt_file(source_path, destination_path, key, "Decryption successful."),
        _ => Err(Error::new(ErrorKind::Other, "Invalid operation. Use 'encrypt' or 'decrypt'.")),
    }
}

fn encrypt_or_decrypt_file(source_file_path: &str, destination_file_path: &str, key: &[u8], success_message: &str) -> io::Result<()> {
    let content = fs::read(source_file_path)?;
    let processed_content = encryption::xor_operation(&content, key);
    fs::write(destination_file_path, processed_content)?;
    println!("{}", success_message);
    Ok(())
}

fn change_file_extension_based_on_operation(operation: &str, file_path: &str) -> String {
    let mut path = PathBuf::from(file_path);

    match operation {
        "encrypt" => path.set_extension("encrypted"),
        "decrypt" => path.set_extension("decrypted"),
        _ => {}
    };

    path.to_str().unwrap_or(file_path).to_string()
}

mod encryption {
    pub fn xor_operation(data: &[u8], key: &[u8]) -> Vec<u8> {
        data.iter()
            .enumerate()
            .map(|(index, &byte)| byte ^ key[index % key.len()])
            .collect()
    }
}