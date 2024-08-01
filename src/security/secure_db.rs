use crate::views;
use std::process::Command;

pub fn encrypt_db(db_folder: &str, db_name: &str) {
    let folder = std::env::current_dir()
        .expect("\x1b[0m\x1b[31;3mFailed to access current directory.\x1b[0m\n");
    let to_encrypt = format!("{}{}{}", folder.display(), db_folder, db_name);
    Command::new("gpg")
        .args([
            "--no-symkey-cache",
            "--symmetric",
            "--cipher-algo",
            "AES256",
            "--yes",
            to_encrypt.as_str(),
        ])
        .output()
        .expect("\x1b[0m\x1b[31;3mFailed to encrypt database.\x1b[0m\n");
    views::start_menus("Database encrypted!");
}

pub fn decrypt_db(db_folder: &str, db_name: &str) {
    let folder = std::env::current_dir()
        .expect("\x1b[0m\x1b[31;3mFailed to access current directory.\x1b[0m\n");
    let output = format!("{}{}{}", folder.display(), db_folder, db_name);
    let to_decrypt = output.clone() + ".gpg";
    Command::new("gpg")
        .args([
            "--no-symkey-cache",
            "--decrypt",
            "--cipher-algo",
            "AES256",
            "-o",
            output.as_str(),
            to_decrypt.as_str(),
        ])
        .output()
        .expect("\x1b[0m\x1b[31;3mFailed to encrypt database.\x1b[0m\n");
    views::start_menus("Database decrypted!");
}
