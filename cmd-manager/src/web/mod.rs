use crate::utils;
mod download;
mod web_req;

pub fn main() {
    utils::clear_screen();
    start_menu();
    'main_loop: loop {
        println!("Enter your option: ");
        let user_input = utils::get_user_input();

        match user_input.trim() {
            "1" => download::main(),
            "2" => web_req::main(),
            _ => {
                if utils::exit_program(&user_input) {
                    break 'main_loop;
                }
                utils::clear_screen()
            }
        }
        start_menu();
    }
    utils::clear_screen();
}

fn start_menu() {
    println!("###############################################");
    println!("#                                             #");
    println!("#                 Web Request                 #");
    println!("#                                             #");
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
