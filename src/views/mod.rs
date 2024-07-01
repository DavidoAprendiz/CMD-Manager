use crate::utils;

/// Main menu
pub fn start_menu_main() {
    let close = "\x1b[0m";

    let blue = format!("{close}\x1b[34m");
    let turquoise_underline = format!("{close}\x1b[36;4m");
    let turquoise_underline_bold = format!("{close}\x1b[36;4m");

    println!("{blue}###############################################");
    println!("#                                             #");
    println!("#           {turquoise_underline_bold}Welcome to CMD-Manager!{blue}           #");
    println!("#                                             #");
    println!("###############################################");
    println!("#                                             #");
    println!(
        "#     {turquoise_underline}1{blue} -> {turquoise_underline}ToDo Manager{blue}                       #"
    );
    println!(
        "#     {turquoise_underline}2{blue} -> {turquoise_underline}File Manager{blue}                       #"
    );
    println!(
        "#     {turquoise_underline}3{blue} -> {turquoise_underline}Web Manager{blue}                        #"
    );
    println!("#                                             #");
    println!(
        "#     {turquoise_underline}Q{blue} -> {turquoise_underline}Exit{blue}                               #"
    );
    println!("###############################################{close}");
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
