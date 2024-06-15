use crate::utils;
use std::fs;

pub fn main() {
    utils::clear_screen();
    start_menu();
    get_pattern();
}

/// Run the menu layout.
fn start_menu() {
    println!("###############################################");
    println!("#                                             #");
    println!("#               Search in File!               #");
    println!("#                                             #");
    println!("###############################################");
}

fn get_pattern() {
    println!("-Please insert the pattern you want to search:");
    get_file(utils::get_user_input())
}

fn get_file(user_pattern: String) {
    println!("\n-Please insert the file path:");
    let user_input = utils::get_user_input();
    get_results(user_pattern, user_input)
}

fn get_results(user_pattern: String, user_file: String) {
    let path = format!(
        "{}\\{user_file}",
        std::env::current_dir()
            .expect("Failed to access current directory.\n")
            .display(),
    );
    println!("###############################################");
    println!("\n- The results for file -> {}\n", &path);
    println!("###############################################");
    let mut count_total: u64 = 0;
    let mut count: u64 = 0;
    match fs::read_to_string(path) {
        Ok(lines) => {
            for line in lines.lines() {
                count += 1;
                if line.trim().contains(user_pattern.trim()) {
                    count_total += 1;
                    println!("Found in line {count}:    {}\n", line.trim());
                }
            }
            println!("Found {count_total} occurrences in total.");
            println!("###############################################\n");
        }
        Err(e) => println!("Failed to open file.\n{e}"),
    }
}
