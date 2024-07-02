use crate::{todo, utils, views};
use std::fs;

const CLOSE: &str = utils::CLOSE;
const BLUE: &str = utils::BLUE;
const CYAN_UNDERLINE: &str = utils::CYAN_UNDERLINE;
const CYAN_UNDERLINE_BOLD: &str = utils::CYAN_UNDERLINE_BOLD;
const ERRO: &str = utils::ERRO;

/// Delete the task specified by the user.
pub fn remove_task() {
    utils::clear_screen();
    views::start_menu_todo_remove();
    utils::get_files_from_folder(todo::tasks_folder());

    println!("{BLUE}###############################################{CLOSE}");
    println!("  {BLUE}Insert the name of the file to be removed{CLOSE}  ");
    println!(
        "     {BLUE}example:{CLOSE}   {CYAN_UNDERLINE_BOLD}16-06-2024-04_09_54.txt{CLOSE}      \n"
    );

    let user_input = utils::get_user_input();

    let path: String = {
        format_args!(
            "{}{}{}",
            std::env::current_dir()
                .expect("\x1b[0m\x1b[31;3mFailed to access current directory.\x1b[0m\n")
                .display(),
            todo::tasks_folder(),
            user_input
        )
        .to_string()
    };

    let file = fs::remove_file(path.trim());
    match file {
        Ok(_) => println!("\n{CYAN_UNDERLINE}File removed:{CLOSE} {BLUE}{path}{CLOSE}\n"),
        Err(e) => println!("\n{ERRO}Failed to remove file.\n{e}{CLOSE}\n"),
    }
}
