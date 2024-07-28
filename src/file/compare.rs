use crate::{utils::*, views};
use similar::TextDiff;
use std::fs;
extern crate similar;

///
/// Compare
///
/// Start menu layout, begin loop, ask user input or exit program
pub fn main() {
    views::start_menus("Compare Files!");
    get_files()
}

fn get_files() {
    get_files_from_folder("".to_string());
    println!("{CYAN_UNDERLINE}First file name:{CLOSE}");
    let file_1 = get_user_input();
    println!("{CYAN_UNDERLINE}Second file name to compare:{CLOSE}");
    let file_2 = get_user_input();
    get_results(file_1, file_2);
}

fn get_results(file1: String, file2: String) {
    let file1_path = get_file_path(file1);
    let file2_path = get_file_path(file2);

    let file_1 = fs::read_to_string(file1_path);
    let file_2 = fs::read_to_string(file2_path);

    print!(
        "{}",
        TextDiff::from_lines(
            &file_1.expect("\x1b[0m\x1b[31;3mFailed to read file 1.\x1b[0m"),
            &file_2.expect("\x1b[0m\x1b[31;3mFailed to read file 2.\x1b[0m"),
        )
        .unified_diff()
        .context_radius(7)
        .header(
            "\x1b[0m\x1b[34mOLD FILE:\n###########################\x1b[0m",
            "\x1b[0m\x1b[34mNEW_FILE:\n###########################\x1b[0m"
        )
    );
    println!("{BLUE}Press ENTER to continue...{CLOSE}");
    get_user_input();
    clear_screen()
}
