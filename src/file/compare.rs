use crate::utils;
use similar::TextDiff;
use std::fs;
extern crate similar;

pub fn main() {
    start_menu();
    get_files()
}

/// Run the menu layout.
fn start_menu() {
    utils::clear_screen();
    println!("###############################################");
    println!("#                                             #");
    println!("#                Compare Files!               #");
    println!("#                                             #");
    println!("###############################################");
}

fn get_files() {
    utils::get_files_from_folder("".to_string());
    println!("- First file:");
    let file_1 = utils::get_user_input();
    println!("- Second file to compare:");
    let file_2 = utils::get_user_input();
    get_results(file_1, file_2);
}

fn get_results(file1: String, file2: String) {
    let file1_path = utils::get_file_path(file1);
    let file2_path = utils::get_file_path(file2);

    let file_1 = fs::read_to_string(file1_path);
    let file_2 = fs::read_to_string(file2_path);

    print!(
        "{}",
        TextDiff::from_lines(
            &file_1.expect("Failed to read file 1."),
            &file_2.expect("Failed to read file 2."),
        )
        .unified_diff()
        .context_radius(7)
        .header(
            "OLD FILE:\n###########################",
            "NEW_FILE:\n###########################"
        )
    );
    // Just to hold the result until key is pressed...
    utils::get_user_input();
    utils::clear_screen()
}
