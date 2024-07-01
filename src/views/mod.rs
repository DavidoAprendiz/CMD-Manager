use crate::utils;

/// Main menu
pub fn start_menu_main() {
    println!("###############################################");
    println!("#                                             #");
    println!("#           Welcome to CMD-Manager!           #");
    println!("#                                             #");
    println!("###############################################");
    println!("###############################################");
    println!("#                                             #");
    println!("#  Select an operation:                       #");
    println!("#                                             #");
    println!("#     '1' -> ToDo Manager                     #");
    println!("#     '2' -> File Manager                     #");
    println!("#     '3' -> Web Manager                      #");
    println!("#                                             #");
    println!("#     'Q' -> Exit                             #");
    println!("###############################################");
}

/// File menu
pub fn start_menu_file() {
    utils::clear_screen();
    println!("###############################################");
    println!("#                                             #");
    println!("#                File Manager                 #");
    println!("#                                             #");
    println!("###############################################");
    println!("###############################################");
    println!("#                                             #");
    println!("#  Select an operation:                       #");
    println!("#                                             #");
    println!("#     '1' -> Search in File                   #");
    println!("#     '2' -> Compare two files                #");
    println!("#                                             #");
    println!("#     'Q' -> Exit                             #");
    println!("###############################################");
}

/// File/Compare menu
pub fn start_menu_compare() {
    utils::clear_screen();
    println!("###############################################");
    println!("#                                             #");
    println!("#                Compare Files!               #");
    println!("#                                             #");
    println!("###############################################");
}

/// File/Search menu
pub fn start_menu_search() {
    utils::clear_screen();
    println!("###############################################");
    println!("#                                             #");
    println!("#               Search in File!               #");
    println!("#                                             #");
    println!("###############################################");
}

/// Web menu
pub fn start_menu_web() {
    utils::clear_screen();
    println!("###############################################");
    println!("#                                             #");
    println!("#                 Web Manager                 #");
    println!("#                                             #");
    println!("###############################################");
    println!("###############################################");
    println!("#                                             #");
    println!("#  Select an operation:                       #");
    println!("#                                             #");
    println!("#     '1' -> Download files                   #");
    println!("#     '2' -> Get price data                   #");
    println!("#                                             #");
    println!("#     'Q' -> Exit                             #");
    println!("###############################################");
}

/// Web/Downloads menu
pub fn start_menu_download() {
    utils::clear_screen();
    println!("###############################################");
    println!("#                                             #");
    println!("#                Download data!               #");
    println!("#                                             #");
    println!("###############################################\n");
    println!("Enter the url to download:");
}

// Web/WebRequests menu
pub fn start_menu_webreq() {
    utils::clear_screen();
    println!("###############################################");
    println!("#                                             #");
    println!("#                 Price data!                 #");
    println!("#                                             #");
    println!("###############################################");
    println!("###############################################");
    println!("#                                             #");
    println!("#  Select an operation:                       #");
    println!("#                                             #");
    println!("#     '1' -> Get Ergo Price                   #");
    println!("#     '2' -> Get Cardano Price                #");
    println!("###############################################");
}
