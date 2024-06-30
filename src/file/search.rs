use crate::utils;
use std::fs;

pub fn main() {
    start_menu();
    get_pattern();
}

/// Run the menu layout.
fn start_menu() {
    utils::clear_screen();
    println!("###############################################");
    println!("#                                             #");
    println!("#               Search in File!               #");
    println!("#                                             #");
    println!("###############################################");
}

fn get_pattern() {
    println!("\n- Available files:\n");
    utils::get_files_from_folder("".to_string());
    println!("\n- Please insert the pattern you want to search:");
    get_file(utils::get_user_input())
}

fn get_file(user_pattern: String) {
    println!("\n- Please insert the file name: (without quotes)");
    let user_input = utils::get_user_input();
    get_results(user_pattern, user_input)
}

fn get_results(user_pattern: String, user_file: String) {
    let path = utils::get_file_path(user_file);

    println!("\n###############################################");
    println!("- The results for  '{}'  in  '{}'\n", &user_pattern, &path);

    let mut count_total: u64 = 0;
    let mut count_line: u64 = 0;
    match fs::read_to_string(path) {
        Ok(lines) => {
            for line in lines.lines() {
                count_line += 1;
                if line.trim().contains(user_pattern.trim()) {
                    count_total += 1;
                    println!("  - Found in line {count_line}:    {}\n", line.trim());
                }
            }
            println!("- {count_total} occurrences in total.");
            println!("###############################################\n");
        }
        Err(e) => println!("Failed to open file.\n{e}"),
    }
    utils::get_user_input();
    utils::clear_screen()
}
