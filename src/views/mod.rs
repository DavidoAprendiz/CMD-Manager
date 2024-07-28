use crate::utils;

// Define constants for colors (change colors in utils.rs)
const CLOSE: &str = utils::CLOSE;
const BLUE: &str = utils::BLUE;
const CYAN_UNDERLINE: &str = utils::CYAN_UNDERLINE;
const CYAN_UNDERLINE_BOLD: &str = utils::CYAN_UNDERLINE_BOLD;

/// Function to build 'simple' menus
pub fn start_menus(show_string: &str) {
    utils::clear_screen();
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
    println!(
        "#     {CYAN_UNDERLINE}5{BLUE} -> {CYAN_UNDERLINE}Security Manager{BLUE}                   #"
    );
    println!("#                                             #");
    println!(
        "#     {CYAN_UNDERLINE}E{BLUE} -> {CYAN_UNDERLINE}Exit{BLUE}                               #"
    );
    println!("###############################################{CLOSE}");
}

///
///  .FILE MANAGER MENU
///
pub fn start_menu_file() {
    start_menus("File Manager!");
    println!("{BLUE}#                                             #");
    println!("#     {CYAN_UNDERLINE}1{BLUE} -> {CYAN_UNDERLINE}Search in File{BLUE}                     #");
    println!("#     {CYAN_UNDERLINE}2{BLUE} -> {CYAN_UNDERLINE}Compare two files{BLUE}                  #");
    println!("#                                             #");
    println!("#     {CYAN_UNDERLINE}E{BLUE} -> {CYAN_UNDERLINE}Exit{BLUE}                               #");
    println!("###############################################{CLOSE}");
}

///
///  .TODO MANAGER MENU
///
pub fn start_menu_todo() {
    start_menus("To-do Manager!");
    println!("{BLUE}#                                             #");
    println!("#     {CYAN_UNDERLINE}1{BLUE} -> {CYAN_UNDERLINE}New Task{BLUE}                           #");
    println!("#     {CYAN_UNDERLINE}2{BLUE} -> {CYAN_UNDERLINE}Remove Task{BLUE}                        #");
    println!("#     {CYAN_UNDERLINE}3{BLUE} -> {CYAN_UNDERLINE}Search Task{BLUE}                        #");
    println!("#     {CYAN_UNDERLINE}4{BLUE} -> {CYAN_UNDERLINE}Show All Tasks{BLUE}                     #");
    println!("#                                             #");
    println!("#     {CYAN_UNDERLINE}E{BLUE} -> {CYAN_UNDERLINE}Exit{BLUE}                               #");
    println!("###############################################{CLOSE}\n");
}

///
///  SECURITY MANAGER MENU
///
pub fn start_menu_security() {
    start_menus("Security Manager!");
    println!("{BLUE}#                                             #");
    println!("#     {CYAN_UNDERLINE}1{BLUE} -> {CYAN_UNDERLINE}Show all security logs{BLUE}             #");
    println!("#                                             #");
    println!("#     {CYAN_UNDERLINE}E{BLUE} -> {CYAN_UNDERLINE}Exit{BLUE}                               #");
    println!("###############################################{CLOSE}\n");
}

///
///  .WEB MANAGER MENU
///
pub fn start_menu_web() {
    start_menus("Web Manager!");
    println!("{BLUE}#                                             #");
    println!("#     {CYAN_UNDERLINE}1{BLUE} -> {CYAN_UNDERLINE}Download files{BLUE}                     #");
    println!("#     {CYAN_UNDERLINE}2{BLUE} -> {CYAN_UNDERLINE}Get price data{BLUE}                     #");
    println!("#                                             #");
    println!("#     {CYAN_UNDERLINE}E{BLUE} -> {CYAN_UNDERLINE}Exit{BLUE}                               #");
    println!("###############################################{CLOSE}");
}

// Web/WebRequests menu
pub fn start_menu_web_request() {
    start_menus("Price data!");
    println!("{BLUE}#                                             #");
    println!("#     {CYAN_UNDERLINE}1{BLUE} -> {CYAN_UNDERLINE}Get Ergo Price{BLUE}                     #");
    println!("#     {CYAN_UNDERLINE}2{BLUE} -> {CYAN_UNDERLINE}Get Cardano Price{BLUE}                  #");
    println!("#                                             #");
    println!("#     {CYAN_UNDERLINE}E{BLUE} -> {CYAN_UNDERLINE}Exit{BLUE}                               #");
    println!("###############################################{CLOSE}");
}

///
/// .BRAIN MANAGER MENU
///
pub fn start_menu_brain() {
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
    println!("#                                             #");
    println!("#     {CYAN_UNDERLINE}E{BLUE} -> {CYAN_UNDERLINE}Exit{BLUE}                               #");
    println!("###############################################{CLOSE}");
}
