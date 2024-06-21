use crate::utils;
use reqwest::blocking::get;
use std::io::Write;
use std::{env, fs};

pub fn main() {
    menu();
    create_folder();
    println!("Enter the url to download: ");
    let user_input = utils::get_user_input();

    let url_response = get_response(user_input);
    write_file(url_response);

    println!("Finished!");
    utils::get_user_input();
}

fn menu() {
    utils::clear_screen();
    println!("###############################################");
    println!("#                Download data!               #");
    println!("###############################################");
}

fn create_folder() {
    match std::fs::create_dir_all(format!(
        "{}/Downloads",
        std::env::current_dir()
            .expect("Failed to access current directory.\n")
            .display(),
    )) {
        Ok(_) => "".to_string(),
        Err(_) => "Failed to create folder 'Tasks'".to_string(),
    };
}
fn get_response(user_input: String) -> String {
    get(user_input)
        .expect("Failed to process request.")
        .text()
        .expect("Failed to display json.")
}

fn write_file(url_response: String) {
    let operating_system = env::consts::OS;
    let operating_system = if operating_system.contains("windows") {
        "\\Downloads\\"
    } else {
        "/Downloads/"
    };

    let file_name = format!(
        "{}{}{}",
        std::env::current_dir()
            .expect("Failed to access current directory.\n")
            .display(),
        operating_system,
        utils::get_current_time()
    );
    let mut file = fs::File::create(file_name).expect("Failed to create text file!\n");
    match file.write_fmt(format_args!("{}", url_response)) {
        Ok(_) => println!("File created!"),
        Err(e) => println!("Failed to save task in file!\n{e}\n"),
    }
}
