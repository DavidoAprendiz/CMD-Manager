use crate::{utils, views};
use std::fs;

/// Search
///
/// Start menu layout, start function 'get_pattern()'. The output is sent to 'get_file() and then 'results()'
pub fn main() {
    views::start_menus("Search in File!");
    println!(
        "\n{}Available files:{}\n",
        utils::CYAN_UNDERLINE,
        utils::CLOSE
    );
    utils::get_files_from_folder("".to_string());
    get_pattern();
    println!("{}Press ENTER to continue...{}", utils::BLUE, utils::CLOSE);
    utils::get_user_input();
}

/// Get word pattern from user
fn get_pattern() {
    println!(
        "\n{}Please insert the pattern you want to search:{}",
        utils::CYAN_UNDERLINE,
        utils::CLOSE
    );
    get_file(utils::get_user_input())
}

/// Get file name from user
fn get_file(user_pattern: String) {
    println!(
        "\n{}Please insert the file name: {}(without quotes){}",
        utils::CYAN_UNDERLINE,
        utils::CYAN_UNDERLINE_BOLD,
        utils::CLOSE
    );
    let user_input = utils::get_user_input();
    get_results(user_pattern, user_input)
}

/// Show results from the chosen pattern and file.
fn get_results(user_pattern: String, user_file: String) {
    let path = utils::get_file_path(user_file);

    println!(
        "\n{}###############################################{}",
        utils::BLUE,
        utils::CLOSE
    );
    println!(
        "{}The results for  '{}'  in  '{}'{}\n",
        utils::CYAN_UNDERLINE,
        user_pattern,
        &path,
        utils::CLOSE
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
            println!(
                "{}{count_total} occurrences in total.{}",
                utils::CYAN_UNDERLINE_BOLD,
                utils::CLOSE
            );
            println!(
                "{}###############################################{}\n",
                utils::BLUE,
                utils::CLOSE
            );
        }
        Err(e) => println!("{}Failed to open file.\n{e}{}", utils::ERRO, utils::CLOSE),
    }
}
