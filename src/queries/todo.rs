use crate::{queries::security, queries::*, utils::*};
use sqlite::State;

///
/// .TODO MANAGER
///

/// Create 'TB_TODO' table
pub fn q_todo_create_table() {
    create_folder(db_folder());
    let path = get_file_path(format!("{}{}", db_folder(), DATABASE));
    let conn = sqlite::open(path).unwrap();

    let query = format!(
        "CREATE TABLE IF NOT EXISTS {TB_TODO} ({CL_TIMESTAMP} TEXT, {CL_TODO_TITLE} TEXT, {CL_TODO_TASK} BLOB);"
    );

    conn.execute(query)
        .expect("\x1b[0m\x1b[31;3mFailed to execute query! 'q_todo_create_table()'\x1b[0m");
}

/// Add user data (Title and Task) to 'TB_TODO' table
pub fn q_todo_add_task(task_name: String, task: String) {
    security::q_security_add_security_timestamps(ADD_TASK);
    let path = get_file_path(format!("{}{}", db_folder(), DATABASE));
    let conn = sqlite::open(path).unwrap();

    let query = format!(
        "
    INSERT INTO {TB_TODO} ({CL_TIMESTAMP}, {CL_TODO_TITLE}, {CL_TODO_TASK}) 
    VALUES (datetime(CURRENT_TIMESTAMP,'localtime'), '{:?}', '{:?}' );",
        r#task_name, r#task
    );

    conn.execute(query)
        .expect("\x1b[0m\x1b[31;3mFailed to execute query! 'q_todo_add_task()'\x1b[0m");
}

/// Delete Todo Tasks
pub fn q_todo_delete_task() {
    security::q_security_add_security_timestamps(REMOVE_TASK);
    let path = get_file_path(format!("{}{}", db_folder(), DATABASE));
    let conn = sqlite::open(path).unwrap();

    println!("\n{CYAN_UNDERLINE}Please insert 'ID' to delete:\n{BLUE}(or press ENTER to continue){CLOSE}");
    let task_id = get_user_input();

    if !task_id.trim().is_empty() && task_id.chars().next().unwrap().is_ascii_digit() {
        let query = format!(
            "
                DELETE FROM {TB_TODO} where {CL_ID} = {:?}; VACUUM;
                ",
            task_id.trim().parse::<i32>().unwrap()
        );
        match conn.execute(query) {
            Ok(_) => {
                println!("{CYAN_UNDERLINE}Task id '{task_id}' was successfully deleted.{CLOSE}")
            }
            Err(_) => println!("{ERRO}Failed to execute query! 'q_todo_delete_task()'{CLOSE}"),
        }
        println!("\n{BLUE}###############################################\nPress ENTER to continue...{CLOSE}");
        get_user_input();
    }
}

/// Search keyword/s in database.
pub fn q_todo_search_keyword(user_input: String) {
    security::q_security_add_security_timestamps(SEARCH_TODO);

    let path = get_file_path(format!("{}{}", db_folder(), DATABASE));
    let conn = sqlite::open(path).unwrap();
    let query =
        format!("SELECT {CL_ID}, {CL_TIMESTAMP}, {CL_TODO_TITLE}, {CL_TODO_TASK} FROM {TB_TODO} WHERE {CL_TODO_TITLE} LIKE '%{user_input}%' OR {CL_TODO_TASK} LIKE '%{user_input}%';");
    let mut statement = conn.prepare(query).unwrap();

    statement.iter();

    println!("\n{BLUE}###############################################{CLOSE}\n");
    println!(
        "\n{CYAN_UNDERLINE}Table{CLOSE}   : {TB_TODO}\n{CYAN_UNDERLINE}Columns{CLOSE} : {CL_ID} | {CL_TIMESTAMP} | {CL_TODO_TITLE} | {CL_TODO_TASK}\n\n");

    while let Ok(State::Row) = statement.next() {
        println!(
            "\n> {} | {} | {} | {}\n",
            statement.read::<String, _>(CL_ID).unwrap(),
            statement.read::<String, _>(CL_TIMESTAMP).unwrap(),
            statement.read::<String, _>(CL_TODO_TITLE).unwrap(),
            statement.read::<String, _>(CL_TODO_TASK).unwrap(),
        );
    }
}

/// Show all data from 'TB_TODO' table
pub fn q_todo_show_all() {
    security::q_security_add_security_timestamps(VIEW_TODO);
    let path = get_file_path(format!("{}{}", db_folder(), DATABASE));
    let conn = sqlite::open(path).unwrap();
    let query =
        format!("SELECT {CL_ID}, {CL_TIMESTAMP}, {CL_TODO_TITLE}, {CL_TODO_TASK}  FROM {TB_TODO};");
    let mut statement = conn.prepare(query).unwrap();
    statement.iter();

    println!("\n{CYAN_UNDERLINE}Table{CLOSE}   : {TB_TODO}\n{CYAN_UNDERLINE}Columns{CLOSE} : {CL_ID} | {CL_TIMESTAMP} | {CL_TODO_TITLE} | {CL_TODO_TASK}\n");
    while let Ok(State::Row) = statement.next() {
        println!(
            "\n> {} | {} | {} | {}",
            statement.read::<String, _>(CL_ID).unwrap(),
            statement.read::<String, _>(CL_TIMESTAMP).unwrap(),
            statement.read::<String, _>(CL_TODO_TITLE).unwrap(),
            statement.read::<String, _>(CL_TODO_TASK).unwrap(),
        );
    }
}
