mod compare;
mod search;

use crate::utils;

pub fn main() {
    utils::clear_screen();
    start_menu();
    'main_loop: loop {
        println!("Enter your option: ");
        let user_input = utils::get_user_input();

        match user_input.trim() {
            "1" => search::main(),
            "2" => compare::main(),
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

/// Run the menu layout.
fn start_menu() {
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
