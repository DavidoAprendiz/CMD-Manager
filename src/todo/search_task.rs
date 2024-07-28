use crate::{queries, utils::*, views};

pub fn search_task() {
    views::start_menus("Search your tasks!");
    println!("\n{CYAN_UNDERLINE}Please insert the pattern you want to search:{CLOSE}");
    let user_input = get_user_input();
    queries::todo::q_todo_search_keyword(user_input);
    println!(
        "{BLUE}###############################################\nPress ENTER to continue...{CLOSE}"
    );
    get_user_input();
}
