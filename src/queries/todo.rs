use crate::{
    queries::{
        security, start_db_connection, ADD_TASK, CL_ID, CL_TIMESTAMP, CL_TODO_TASK, CL_TODO_TITLE,
        REMOVE_TASK, SEARCH_TODO, TB_TODO, VIEW_TODO,
    },
    utils::{get_user_input, BLUE, CLOSE, CYAN_UNDERLINE, ERRO},
};
use sqlite::State;

///
/// .TODO MANAGER
///

/// Add user data (Title and Task) to 'TB_TODO' table
pub fn q_todo_add_task(task_name: String, task: String) {
    security::q_security_add_timestamps(ADD_TASK);
    let conn = start_db_connection();
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
    security::q_security_add_timestamps(REMOVE_TASK);
    let conn = start_db_connection();
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
    security::q_security_add_timestamps(SEARCH_TODO);
    let conn = start_db_connection();
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
    security::q_security_add_timestamps(VIEW_TODO);
    let conn = start_db_connection();
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
