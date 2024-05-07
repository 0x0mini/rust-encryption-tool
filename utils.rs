use std::env;
use std::fs;
use std::path::Path;
use std::io::{self, Write, Read};

pub fn is_valid_file_path(file_path: &str) -> Result<(), String> {
    let path_obj = Path::new(file_path);
    if path_obj.exists() && path_obj.is_file() {
        Ok(())
    } else {
        Err(format!("Path does not exist or is not a file: {}", file_path))
    }
}

pub fn check_file_readable(file_path: &str) -> Result<(), String> {
    match fs::File::open(file_path) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Cannot read file: {}. Error: {}", file_path, e)),
    }
}

pub fn check_file_writable(file_path: &str) -> Result<(), String> {
    match fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_path) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Cannot write to file: {}. Error: {}", file_path, e)),
    }
}

pub fn log_information(message: &str) {
    match env::var("LOG_PATH") {
        Ok(log_file_path) => {
            match fs::OpenOptions::new().append(true).create(true).open(&log_file_path) {
                Ok(mut file) => {
                    if let Err(e) = writeln!(file, "INFO: {}", message) {
                        eprintln!("Failed to write to log file at {}. Error: {}", log_file_path, e);
                    }
                },
                Err(e) => {
                    eprintln!("Failed to open log file at {}. Error: {}", log_file_path, e);
                }
            };
        },
        Err(_) => println!("INFO: {}", message),
    }
}

pub fn log_error_message(message: &str) {
    match env::var("LOG_PATH") {
        Ok(log_file_path) => {
            match fs::OpenOptions::new().append(true).create(true).open(&log_file_path) {
                Ok(mut file) => {
                    if let Err(e) = writeln!(file, "ERROR: {}", message) {
                        eprintln!("Failed to write error to log file at {}. Error: {}", log_file_path, e);
                    }
                },
                Err(e) => {
                    eprintln!("Failed to open log file for error logging at {}. Error: {}", log_file_path, e);
                }
            };
        },
        Err(_) => eprintln!("ERROR: {}", message),
    }
}

pub fn encrypt_decrypt_file(file_path: &str, key: u8) -> Result<(), String> {
    let path = Path::new(file_path);

    if !path.exists() || !path.is_file() {
        return Err(format!("File does not exist or is not a proper file: {}", file_path));
    }

    let mut file_content = match fs::read(file_path) {
        Ok(content) => content,
        Err(e) => return Err(format!("Failed to read file: {}. Error: {}", file_path, e)),
    };

    let encrypted_decrypted_data: Vec<u8> = file_content.into_iter().map(|byte| byte ^ key).collect();

    match fs::write(file_path, encrypted_decrypted_data) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to write back to file: {}. Error: {}", file_path, e)),
    }
}