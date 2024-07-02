use crate::utils;

const CLOSE: &str = utils::CLOSE;
const BLUE: &str = utils::BLUE;
const CYAN_UNDERLINE: &str = utils::CYAN_UNDERLINE;
const CYAN_UNDERLINE_BOLD: &str = utils::CYAN_UNDERLINE_BOLD;

/// Main menu
pub fn start_menu_main() {
    println!("{BLUE}###############################################");
    println!("#                                             #");
    println!("#           {CYAN_UNDERLINE_BOLD}Welcome to CMD-Manager!{BLUE}           #");
    println!("#                                             #");
    println!("###############################################");
    println!("#                                             #");
    println!(
        "#     {CYAN_UNDERLINE}1{BLUE} -> {CYAN_UNDERLINE}ToDo Manager{BLUE}                       #"
    );
    println!(
        "#     {CYAN_UNDERLINE}2{BLUE} -> {CYAN_UNDERLINE}File Manager{BLUE}                       #"
    );
    println!(
        "#     {CYAN_UNDERLINE}3{BLUE} -> {CYAN_UNDERLINE}Web Manager{BLUE}                        #"
    );
    println!("#                                             #");
    println!(
        "#     {CYAN_UNDERLINE}Q{BLUE} -> {CYAN_UNDERLINE}Exit{BLUE}                               #"
    );
    println!("###############################################{CLOSE}");
}

//
//  .FILE MANAGER
//

/// File menu
pub fn start_menu_file() {
    utils::clear_screen();
    println!("{BLUE}###############################################");
    println!("#                                             #");
    println!("#                {CYAN_UNDERLINE_BOLD}File Manager{BLUE}                 #");
    println!("#                                             #");
    println!("###############################################");
    println!("#                                             #");
    println!("#     {CYAN_UNDERLINE}1{BLUE} -> {CYAN_UNDERLINE}Search in File{BLUE}                     #");
    println!("#     {CYAN_UNDERLINE}2{BLUE} -> {CYAN_UNDERLINE}Compare two files{BLUE}                  #");
    println!("#                                             #");
    println!("#     {CYAN_UNDERLINE}Q{BLUE} -> {CYAN_UNDERLINE}Exit{BLUE}                               #");
    println!("###############################################{CLOSE}");
}

/// File/Compare menu
pub fn start_menu_compare() {
    utils::clear_screen();
    println!("{BLUE}###############################################");
    println!("#                                             #");
    println!("#                {CYAN_UNDERLINE_BOLD}Compare Files!{BLUE}               #");
    println!("#                                             #");
    println!("###############################################{CLOSE}");
}

/// File/Search menu
pub fn start_menu_search() {
    utils::clear_screen();
    println!("{BLUE}###############################################");
    println!("#                                             #");
    println!("#               {CYAN_UNDERLINE_BOLD}Search in File!{BLUE}               #");
    println!("#                                             #");
    println!("###############################################{CLOSE}");
}

//
//  .TODO MANAGER
//

/// Todo menu
pub fn start_menu_todo() {
    utils::clear_screen();
    println!("{BLUE}###############################################");
    println!("#                                             #");
    println!("#                {CYAN_UNDERLINE_BOLD}To-do Manager{BLUE}                #");
    println!("#                                             #");
    println!("###############################################");
    println!("#                                             #");
    println!("#     {CYAN_UNDERLINE}1{BLUE} -> {CYAN_UNDERLINE}New Task{BLUE}                           #");
    println!("#     {CYAN_UNDERLINE}2{BLUE} -> {CYAN_UNDERLINE}Remove Task{BLUE}                        #");
    println!("#     {CYAN_UNDERLINE}3{BLUE} -> {CYAN_UNDERLINE}Show All Tasks{BLUE}                     #");
    println!("#     {CYAN_UNDERLINE}4{BLUE} -> {CYAN_UNDERLINE}View Task{BLUE}                          #");
    println!("#                                             #");
    println!("#     {CYAN_UNDERLINE}Q{BLUE} -> {CYAN_UNDERLINE}Exit{BLUE}                               #");
    println!("###############################################{CLOSE}\n");
}

/// Todo New Task
pub fn start_menu_todo_new() {
    println!("{BLUE}###############################################");
    println!("#              {CYAN_UNDERLINE_BOLD}Create a new task!{BLUE}             #");
    println!("###############################################{CLOSE}\n");
}

/// Todo Task Saved
pub fn start_menu_todo_saved() {
    println!("{BLUE}###############################################");
    println!("#                 {CYAN_UNDERLINE_BOLD}Task saved!{BLUE}                 #");
    println!("###############################################{CLOSE}\n");
}

/// Todo All Saved Tasks
pub fn start_menu_todo_all() {
    println!("{BLUE}###############################################");
    println!("#                  {CYAN_UNDERLINE_BOLD}All Tasks!{BLUE}                 #");
    println!("###############################################{CLOSE}\n");
}

/// Todo View Task
pub fn start_menu_todo_view() {
    println!("{BLUE}###############################################");
    println!("#              {CYAN_UNDERLINE_BOLD}View your tasks!{BLUE}               #");
    println!("###############################################{CLOSE}\n");
}

/// Todo Remove Task
pub fn start_menu_todo_remove() {
    println!("{BLUE}###############################################");
    println!("#                {CYAN_UNDERLINE_BOLD}Remove a task!{BLUE}               #");
    println!("###############################################{CLOSE}\n");
}

//
//  .WEB MANAGER
//

/// Web menu
pub fn start_menu_web() {
    utils::clear_screen();
    println!("{BLUE}###############################################");
    println!("#                                             #");
    println!("#                 {CYAN_UNDERLINE_BOLD}Web Manager{BLUE}                 #");
    println!("#                                             #");
    println!("###############################################");
    println!("#                                             #");
    println!("#     {CYAN_UNDERLINE}1{BLUE} -> {CYAN_UNDERLINE}Download files{BLUE}                     #");
    println!("#     {CYAN_UNDERLINE}2{BLUE} -> {CYAN_UNDERLINE}Get price data{BLUE}                     #");
    println!("#                                             #");
    println!("#     {CYAN_UNDERLINE}Q{BLUE} -> {CYAN_UNDERLINE}Exit{BLUE}                               #");
    println!("###############################################{CLOSE}");
}

/// Web/Downloads menu
pub fn start_menu_download() {
    utils::clear_screen();
    println!("{BLUE}###############################################");
    println!("#                                             #");
    println!("#                {CYAN_UNDERLINE_BOLD}Download data!{BLUE}               #");
    println!("#                                             #");
    println!("###############################################{CLOSE}\n");
}

// Web/WebRequests menu
pub fn start_menu_web_request() {
    utils::clear_screen();
    println!("{BLUE}###############################################");
    println!("#                                             #");
    println!("#                 {CYAN_UNDERLINE_BOLD}Price data!{BLUE}                 #");
    println!("#                                             #");
    println!("###############################################");
    println!("#                                             #");
    println!("#     {CYAN_UNDERLINE}1{BLUE} -> {CYAN_UNDERLINE}Get Ergo Price{BLUE}                     #");
    println!("#     {CYAN_UNDERLINE}2{BLUE} -> {CYAN_UNDERLINE}Get Cardano Price{BLUE}                  #");
    println!("###############################################{CLOSE}");
}
