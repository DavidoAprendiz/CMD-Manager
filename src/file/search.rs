use crate::{utils, views};
use std::fs;

const CLOSE: &str = utils::CLOSE;
const BLUE: &str = utils::BLUE;
const CYAN_UNDERLINE: &str = utils::CYAN_UNDERLINE;
const CYAN_UNDERLINE_BOLD: &str = utils::CYAN_UNDERLINE_BOLD;
const ERRO: &str = utils::ERRO;

pub fn main() {
    views::start_menu_search();
    get_pattern();
}

fn get_pattern() {
    println!("\n{CYAN_UNDERLINE}Available files:{CLOSE}\n");
    utils::get_files_from_folder("".to_string());
    println!("\n{CYAN_UNDERLINE}Please insert the pattern you want to search:{CLOSE}");
    get_file(utils::get_user_input())
}

fn get_file(user_pattern: String) {
    println!("\n{CYAN_UNDERLINE}Please insert the file name: {CYAN_UNDERLINE_BOLD}(without quotes){CLOSE}");
    let user_input = utils::get_user_input();
    get_results(user_pattern, user_input)
}

fn get_results(user_pattern: String, user_file: String) {
    let path = utils::get_file_path(user_file);

    println!("\n{BLUE}###############################################{CLOSE}");
    println!(
        "{CYAN_UNDERLINE}The results for  '{}'  in  '{}'{CLOSE}\n",
        &user_pattern, &path
    );

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
            println!("{CYAN_UNDERLINE_BOLD}{count_total} occurrences in total.{CLOSE}");
            println!("{BLUE}###############################################{CLOSE}\n");
        }
        Err(e) => println!("{ERRO}Failed to open file.\n{e}{CLOSE}"),
    }
    println!("{BLUE}Press ENTER to continue...{CLOSE}");
    utils::get_user_input();
    utils::clear_screen()
}