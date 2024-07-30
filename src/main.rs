mod brain; // Brain Manager
mod file; // File Manager
mod queries; // All Queries to DB
mod security; // Security Manager
mod todo; // Todo Manager
mod utils; // Common functions
mod views; // Application views
mod web; // Web Manager

use queries::{
    CL_ANSWERS, CL_DOWNLOAD_BODY, CL_DOWNLOAD_URL, CL_FILENAME, CL_FILE_CONTENT, CL_QUESTIONS,
    CL_TODO_TASK, CL_TODO_TITLE, CL_WEB_NAME, CL_WEB_PRICE, TB_BRAIN, TB_FILE_SEARCH, TB_TODO,
    TB_WEB_API, TB_WEB_DOWNLOAD,
};

/// Application main loop
///
/// Start menu layout, begin loop, ask user input or exit program
fn main() {
    views::start_menu_main();
    first_config();
    'main_loop: loop {
        println!("Enter your option: ");
        let user_input = utils::get_user_input();
        match user_input.trim() {
            "1" => todo::main(),
            "2" => file::main(),
            "3" => web::main(),
            "4" => brain::main(),
            "5" => security::main(),
            _ => {
                if utils::exit_program(&user_input) {
                    break 'main_loop;
                }
            }
        }
        views::start_menu_main();
    }
}

/// Create all tables (if not exists).
/// Add timestamp/logs to Table SECURITY
fn first_config() {
    utils::create_folder(queries::db_folder());
    queries::security::q_security_create_table();
    queries::file::q_file_compare_create_table();
    queries::q_create_table(TB_FILE_SEARCH, CL_FILENAME, CL_FILE_CONTENT);
    queries::q_create_table(TB_TODO, CL_TODO_TITLE, CL_TODO_TASK);
    queries::q_create_table(TB_BRAIN, CL_QUESTIONS, CL_ANSWERS);
    queries::q_create_table(TB_WEB_API, CL_WEB_NAME, CL_WEB_PRICE);
    queries::q_create_table(TB_WEB_DOWNLOAD, CL_DOWNLOAD_URL, CL_DOWNLOAD_BODY);
    queries::security::q_security_add_timestamps(queries::SYSTEM_LOGON);
}
