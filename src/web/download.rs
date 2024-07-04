use crate::{utils, views};
use reqwest::blocking::get;
use std::{env, fs, io::Write};

// Define constants for colors (change colors in utils.rs)
const CLOSE: &str = utils::CLOSE;
const BLUE: &str = utils::BLUE;
const CYAN_UNDERLINE: &str = utils::CYAN_UNDERLINE;
const ERRO: &str = utils::ERRO;

/// Download
///
/// Check folder Downloads (if !exists and OS), get user input and write directly to file
pub fn main() {
    utils::create_folder(downloads_folder());
    views::start_menu_download();
    println!("{CYAN_UNDERLINE}Enter the url to download:{CLOSE}");
    let user_input = utils::get_user_input();
    let url_response = get_response(user_input);
    write_file(url_response);
    println!("{BLUE}Press ENTER to continue...{CLOSE}");
    utils::get_user_input();
}

/// Create folder Downloads (if does't exist) and verify the running operating system (OS) and return the correct path.
fn downloads_folder() -> String {
    let operating_system = env::consts::OS;
    if operating_system.contains("windows") {
        "\\Project\\Downloads\\".to_string()
    } else {
        "/Project/Downloads/".to_string()
    }
}

/// Generate a response from an URL.
fn get_response(user_input: String) -> String {
    match get(user_input) {
        Ok(text) => text
            .text()
            .expect("\x1b[0m\x1b[31;3mFailed to display json.\x1b[0m"),
        Err(e) => format!("{ERRO}Failed to process request...\n{e}{CLOSE}\n"),
    }
}

/// Write the URL content to a local file
fn write_file(url_response: String) {
    let file_name = format!(
        "{}{}{}",
        std::env::current_dir()
            .expect("\x1b[0m\x1b[31;3mFailed to access current directory.\x1b[0m\n")
            .display(),
        downloads_folder(),
        utils::get_current_time()
    );
    let mut file =
        fs::File::create(file_name).expect("\x1b[0m\x1b[31;3mFailed to create text file!\x1b[0m\n");
    match file.write_fmt(format_args!("{}", url_response)) {
        Ok(_) => println!("\n{CYAN_UNDERLINE}File successfully created!{CLOSE}\n"),
        Err(e) => println!("{ERRO}Failed to save task in file!\n{e}{CLOSE}\n"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _downloads_folder() {
        assert_eq!(
            downloads_folder(),
            if env::consts::OS.contains("windows") {
                "\\Project\\Downloads\\".to_string()
            } else {
                "/Project/Downloads/".to_string()
            }
        );
    }
}
