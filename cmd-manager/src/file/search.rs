use crate::utils;

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
    println!("Please insert the pattern you want to search:");
    get_file(utils::get_user_input())
}

fn get_file(user_pattern: String) {
    println!("\nPlease insert the file path:");
    let user_input = utils::get_user_input();
    get_results(user_pattern, user_input)
}

fn get_results(user_pattern: String, user_file: String) {
    println!("\nThe results:\n");
    println!("{user_pattern}");
    println!("{user_file}");
}
