mod file; // File Manager
mod todo; // Todo Manager
mod utils; // Common functions
mod views; // All views
mod web; // Web Manager // Application Views

fn main() {
    utils::clear_screen();

    views::start_menu_main();

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
        views::start_menu_main();
    }
}
