mod file; // File Manager
mod todo; // Todo Manager
mod utils; // Common functions
mod web; // Web Manager

fn main() {
    utils::clear_screen();
    start_menu();
    'main_loop: loop {
        println!("Enter your option: ");
        let user_input = utils::get_user_input();

        match user_input.trim() {
            "1" => todo::main(),
            "2" => file::main(),
            "3" => web::main(),
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

/// Run the menu layout.
fn start_menu() {
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
