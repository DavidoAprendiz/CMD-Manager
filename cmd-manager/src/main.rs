mod todo;
mod utils;

pub fn main() {
    utils::clear_screen();
    start_menu();
    'main_loop: loop {
        println!("Enter your option: ");
        let user_input = utils::get_user_input();

        match user_input.trim() {
            "1" => todo::main(),
            _ => {
                if utils::exit_program(&user_input) {
                    break 'main_loop;
                }
                utils::clear_screen()
            }
        }
        start_menu();
    }
}

fn start_menu() {
    println!("###############################################");
    println!("#                                             #");
    println!("#           Welcome to cmd-manager!           #");
    println!("#                                             #");
    println!("###############################################");
    println!("###############################################");
    println!("#                                             #");
    println!("#  Select an operation:                       #");
    println!("#                                             #");
    println!("#     '1' -> ToDo Manager                     #");
    println!("#                                             #");
    println!("#     'Q' -> Exit                             #");
    println!("###############################################");
}
