use crate::{utils, views};
use similar::TextDiff;
use std::fs;
extern crate similar;

///
/// Compare
///
/// Start menu layout, begin loop, ask user input or exit program
pub fn main() {
    views::start_menus("Compare Files!");
    utils::get_files_from_folder("".to_string());
    println!("{}First file name:{}", utils::CYAN_UNDERLINE, utils::CLOSE);
    let file_1 = utils::get_user_input();
    println!(
        "{}Second file name to compare:{}",
        utils::CYAN_UNDERLINE,
        utils::CLOSE
    );
    let file_2 = utils::get_user_input();

    get_files(file_1, file_2);
    println!("{}Press ENTER to continue...{}", utils::BLUE, utils::CLOSE);
    utils::get_user_input();
}

fn get_files(file1: String, file2: String) {
    utils::get_files_from_folder("".to_string());
    get_results(file1, file2);
}

fn get_results(file1: String, file2: String) {
    utils::clear_screen();
    let file1_path = utils::get_file_path(file1);
    let file2_path = utils::get_file_path(file2);

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
}
