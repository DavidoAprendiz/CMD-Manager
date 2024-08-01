use crate::utils::{clear_screen, BLUE, CLOSE, CYAN_UNDERLINE, CYAN_UNDERLINE_BOLD};

/// Function to build 'simple' menus
pub fn start_menus(show_string: &str) {
    clear_screen();
    let blank_spaces = 45 - show_string.len();
    let total_blank_spaces = String::from(" ").repeat(blank_spaces / 2);

    println!("{BLUE}###############################################");
    println!("#                                             #");
    if blank_spaces % 2 != 0 {
        println!(
            "#{total_blank_spaces}{CYAN_UNDERLINE_BOLD}{show_string}{BLUE}{}#",
            total_blank_spaces.to_owned() + " "
        );
    } else {
        println!(
            "#{total_blank_spaces}{CYAN_UNDERLINE_BOLD}{show_string}{BLUE}{total_blank_spaces}#",
        );
    }
    println!("#                                             #");
    println!("###############################################{CLOSE}");
}

pub fn start_menu_lines(show_string: &str, index: &str) {
    let blank_spaces = 45 - show_string.len();
    let total_blank_spaces = String::from(" ").repeat(blank_spaces - 10);
    println!(
        "{BLUE}#     {CYAN_UNDERLINE}{index}{BLUE} -> {CYAN_UNDERLINE}{show_string}{BLUE}{total_blank_spaces}#{CLOSE}",
        
    );
}

/// Main menu
pub fn start_menu_main() {
    start_menus("Welcome to CMD-Manager!");
    println!("{BLUE}#                                             #");
    start_menu_lines("ToDo Manager", "1");
    start_menu_lines("File Manager", "2");
    start_menu_lines("Web Manager", "3");
    start_menu_lines("Brain Manager", "4");
    start_menu_lines("Security Manager", "5");
    println!("{BLUE}#                                             #");
    start_menu_lines("Exit", "E");
    println!("{BLUE}###############################################{CLOSE}\n");
}

///
///  .FILE MANAGER MENU
///
pub fn start_menu_file() {
    start_menus("File Manager!");
    println!("{BLUE}#                                             #");
    start_menu_lines("Search in File", "1");
    start_menu_lines("Compare two files", "2");
    println!("{BLUE}#                                             #");
    start_menu_lines("Exit", "E");
    println!("{BLUE}###############################################{CLOSE}\n");

}

///
///  .TODO MANAGER MENU
///
pub fn start_menu_todo() {
    start_menus("To-do Manager!");
    println!("{BLUE}#                                             #");
    start_menu_lines("New Task", "1");
    start_menu_lines("Remove Task", "2");
    start_menu_lines("Search Task", "3");
    start_menu_lines("Show All Tasks", "4");
    println!("{BLUE}#                                             #");
    start_menu_lines("Exit", "E");
    println!("{BLUE}###############################################{CLOSE}\n");

}

///
///  SECURITY MANAGER MENU
///
pub fn start_menu_security() {
    start_menus("Security Manager!");
    println!("{BLUE}#                                             #");
    start_menu_lines("Show all security logs", "1");
    start_menu_lines("Encrypt DB with GPG (symmetric key)", "2");
    start_menu_lines("Decrypt DB with GPG (symmetric key)", "3");
    println!("{BLUE}#                                             #");
    start_menu_lines("Exit", "E");
    println!("{BLUE}###############################################{CLOSE}\n");

}

///
///  .WEB MANAGER MENU
///
pub fn start_menu_web() {
    start_menus("Web Manager!");
    println!("{BLUE}#                                             #");
    start_menu_lines("Download files", "1");
    start_menu_lines("Get price data", "2");
    println!("{BLUE}#                                             #");
    start_menu_lines("Exit", "E");
    println!("{BLUE}###############################################{CLOSE}\n");

}

// Web/WebRequests menu
pub fn start_menu_web_request() {
    start_menus("Price data!");
    println!("{BLUE}#                                             #");
    start_menu_lines("Get Ergo Price", "1");
    start_menu_lines("Get Cardano Price", "2");
    println!("{BLUE}#                                             #");
    start_menu_lines("Exit", "E");
    println!("{BLUE}###############################################{CLOSE}\n");

}

///
/// .BRAIN MANAGER MENU
///
pub fn start_menu_brain() {
    start_menus("Brain Manager!");
    println!("{BLUE}#                                             #");
    start_menu_lines("Start a new conversation", "1");
    start_menu_lines("Search 'keywords' in the DB", "2");
    start_menu_lines("Delete conversations", "3");
    start_menu_lines("Show all conversation history", "4");
    println!("{BLUE}#                                             #");
    start_menu_lines("Exit", "E");
    println!("{BLUE}###############################################{CLOSE}\n");

}
