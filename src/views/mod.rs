use crate::utils;

// Define constants for colors (change colors in utils.rs)
const CLOSE: &str = utils::CLOSE;
const BLUE: &str = utils::BLUE;
const CYAN_UNDERLINE: &str = utils::CYAN_UNDERLINE;
const CYAN_UNDERLINE_BOLD: &str = utils::CYAN_UNDERLINE_BOLD;

/// Function to 'build' all menus
pub fn start_menus(show_string: &str) {
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

/// Main menu
pub fn start_menu_main() {
    utils::clear_screen();
    start_menus("Welcome to CMD-Manager!");
    println!("{BLUE}#                                             #");
    println!(
        "#     {CYAN_UNDERLINE}1{BLUE} -> {CYAN_UNDERLINE}ToDo Manager{BLUE}                       #"
    );
    println!(
        "#     {CYAN_UNDERLINE}2{BLUE} -> {CYAN_UNDERLINE}File Manager{BLUE}                       #"
    );
    println!(
        "#     {CYAN_UNDERLINE}3{BLUE} -> {CYAN_UNDERLINE}Web Manager{BLUE}                        #"
    );
    println!(
        "#     {CYAN_UNDERLINE}4{BLUE} -> {CYAN_UNDERLINE}Brain Manager{BLUE}                      #"
    );
    println!("#                                             #");
    println!(
        "#     {CYAN_UNDERLINE}E{BLUE} -> {CYAN_UNDERLINE}Exit{BLUE}                               #"
    );
    println!("###############################################{CLOSE}");
}

//
//  .FILE MANAGER
//

/// File menu
pub fn start_menu_file() {
    utils::clear_screen();
    start_menus("File Manager");
    println!("{BLUE}#                                             #");
    println!("#     {CYAN_UNDERLINE}1{BLUE} -> {CYAN_UNDERLINE}Search in File{BLUE}                     #");
    println!("#     {CYAN_UNDERLINE}2{BLUE} -> {CYAN_UNDERLINE}Compare two files{BLUE}                  #");
    println!("#                                             #");
    println!("#     {CYAN_UNDERLINE}E{BLUE} -> {CYAN_UNDERLINE}Exit{BLUE}                               #");
    println!("###############################################{CLOSE}");
}

/// File/Compare menu
pub fn start_menu_compare() {
    utils::clear_screen();
    start_menus("Compare Files!");
}

/// File/Search menu
pub fn start_menu_search() {
    utils::clear_screen();
    start_menus("Search in File!");
}

//
//  .TODO MANAGER
//

/// Todo menu
pub fn start_menu_todo() {
    utils::clear_screen();
    start_menus("To-do Manager");
    println!("{BLUE}#                                             #");
    println!("#     {CYAN_UNDERLINE}1{BLUE} -> {CYAN_UNDERLINE}New Task{BLUE}                           #");
    println!("#     {CYAN_UNDERLINE}2{BLUE} -> {CYAN_UNDERLINE}Remove Task{BLUE}                        #");
    println!("#     {CYAN_UNDERLINE}3{BLUE} -> {CYAN_UNDERLINE}View Task{BLUE}                          #");
    println!("#     {CYAN_UNDERLINE}4{BLUE} -> {CYAN_UNDERLINE}Show All Tasks{BLUE}                     #");
    println!("#                                             #");
    println!("#     {CYAN_UNDERLINE}E{BLUE} -> {CYAN_UNDERLINE}Exit{BLUE}                               #");
    println!("###############################################{CLOSE}\n");
}

/// Todo New Task
pub fn start_menu_todo_new() {
    utils::clear_screen();
    start_menus("Create a new task!");
}

/// Todo Task Saved
pub fn start_menu_todo_saved() {
    start_menus("Task saved!");
}

/// Todo All Saved Tasks
pub fn start_menu_todo_all() {
    utils::clear_screen();
    start_menus("Show All Tasks!");
}

/// Todo View Task
pub fn start_menu_todo_view() {
    utils::clear_screen();
    start_menus("View your tasks!");
}

/// Todo Remove Task
pub fn start_menu_todo_remove() {
    utils::clear_screen();
    start_menus("Remove a task!");
}

//
//  .WEB MANAGER
//

/// Web menu
pub fn start_menu_web() {
    utils::clear_screen();
    start_menus("Web Manager");
    println!("{BLUE}#                                             #");
    println!("#     {CYAN_UNDERLINE}1{BLUE} -> {CYAN_UNDERLINE}Download files{BLUE}                     #");
    println!("#     {CYAN_UNDERLINE}2{BLUE} -> {CYAN_UNDERLINE}Get price data{BLUE}                     #");
    println!("#                                             #");
    println!("#     {CYAN_UNDERLINE}E{BLUE} -> {CYAN_UNDERLINE}Exit{BLUE}                               #");
    println!("###############################################{CLOSE}");
}

// Web/WebRequests menu
pub fn start_menu_web_request() {
    utils::clear_screen();
    start_menus("Price data!");
    println!("{BLUE}#                                             #");
    println!("#     {CYAN_UNDERLINE}1{BLUE} -> {CYAN_UNDERLINE}Get Ergo Price{BLUE}                     #");
    println!("#     {CYAN_UNDERLINE}2{BLUE} -> {CYAN_UNDERLINE}Get Cardano Price{BLUE}                  #");
    println!("#                                             #");
    println!("#     {CYAN_UNDERLINE}E{BLUE} -> {CYAN_UNDERLINE}Exit{BLUE}                               #");
    println!("###############################################{CLOSE}");
}

//
// .BRAIN MANAGER
//

// Brain menu
pub fn start_menu_brain() {
    utils::clear_screen();
    start_menus("Brain Manager!");
    println!("{BLUE}#                                             #");
    println!(
        "#     {CYAN_UNDERLINE}1{BLUE} -> {CYAN_UNDERLINE}Start a new conversation{BLUE}           #"
    );
    println!(
        "#     {CYAN_UNDERLINE}2{BLUE} -> {CYAN_UNDERLINE}Search 'keywords' in the DB{BLUE}        #"
    );
    println!(
        "#     {CYAN_UNDERLINE}3{BLUE} -> {CYAN_UNDERLINE}Delete conversations{BLUE}               #"
    );
    println!(
        "#     {CYAN_UNDERLINE}4{BLUE} -> {CYAN_UNDERLINE}Show all conversation history{BLUE}      #"
    );
    println!(
        "#     {CYAN_UNDERLINE}5{BLUE} -> {CYAN_UNDERLINE}Show all security history{BLUE}          #"
    );
    println!("#                                             #");
    println!("#     {CYAN_UNDERLINE}E{BLUE} -> {CYAN_UNDERLINE}Exit{BLUE}                               #");
    println!("###############################################{CLOSE}");
}
