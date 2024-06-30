use crate::utils;
use reqwest::blocking::get;
use std::{env, fs, io::Write};

pub fn main() {
    menu();
    println!("Enter the url to download: ");
    let user_input = utils::get_user_input();
    let url_response = get_response(user_input);
    write_file(url_response);
    println!("Finished!");
    utils::get_user_input();
}

fn menu() {
    utils::clear_screen();
    utils::create_folder(downloads_folder());
    println!("###############################################");
    println!("#                Download data!               #");
    println!("###############################################");
}

fn downloads_folder() -> String {
    let operating_system = env::consts::OS;
    if operating_system.contains("windows") {
        "\\Project\\Downloads\\".to_string()
    } else {
        "/Project/Downloads/".to_string()
    }
}

fn get_response(user_input: String) -> String {
    match get(user_input) {
        Ok(text) => text.text().expect("Failed to display json."),
        Err(e) => format!("Failed to process request...\n{e}"),
    }
}

fn write_file(url_response: String) {
    let file_name = format!(
        "{}{}{}",
        std::env::current_dir()
            .expect("Failed to access current directory.\n")
            .display(),
        downloads_folder(),
        utils::get_current_time()
    );
    let mut file = fs::File::create(file_name).expect("Failed to create text file!\n");
    match file.write_fmt(format_args!("{}", url_response)) {
        Ok(_) => println!("File created!"),
        Err(e) => println!("Failed to save task in file!\n{e}\n"),
    }
}
