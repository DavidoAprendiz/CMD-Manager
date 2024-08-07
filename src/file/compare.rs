use crate::{
    queries::{file, CL_FILENAME_1, CL_FILENAME_2, COMPARE_FILE, TB_FILE_COMPARE},
    utils, views,
};
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

    let file_1: Result<String, std::io::Error> = fs::read_to_string(&file1_path);
    let file_2: Result<String, std::io::Error> = fs::read_to_string(&file2_path);

    println!("FILE 1: {file1_path}");
    println!("FILE 2: {file2_path}");

    let diff: Vec<String> = TextDiff::from_lines(
        &file_1.unwrap_or("\x1b[0m\x1b[31;3mFailed to read files.\x1b[0m".to_string()),
        &file_2.unwrap_or("\x1b[0m\x1b[31;3mFailed to read files.\x1b[0m".to_string()),
    )
    .iter_all_changes()
    .map(|change| format!("{:?} {}", change.tag(), change))
    .collect();

    file::q_file_save_data(
        TB_FILE_COMPARE,
        CL_FILENAME_1,
        CL_FILENAME_2,
        file1_path.as_str(),
        file2_path.as_str(),
        diff.iter()
            .map(|x| x.to_string())
            .collect::<String>()
            .as_str(),
        COMPARE_FILE,
    );

    println!("{}", diff.iter().map(|x| x.to_string()).collect::<String>());
}
