use crate::{queries::security, queries::*, utils::*, views};
use sqlite::State;

///
/// BRAIN MANAGER
///

/// Create 'TB_BRAIN' table
pub fn q_brain_create_table() {
    create_folder(db_folder());
    let path = get_file_path(format!("{}{}", db_folder(), DATABASE));
    let conn = sqlite::open(path).unwrap();

    let query = format!(
        "CREATE TABLE IF NOT EXISTS {TB_BRAIN} ({CL_TIMESTAMP} TEXT, {CL_QUESTIONS} BLOB, {CL_ANSWERS} BLOB);"
    );

    conn.execute(query)
        .expect("\x1b[0m\x1b[31;3mFailed to execute query! 'q_brain_create_table()'\x1b[0m");
}

/// Insert data 'timestamp',user 'question' and user 'answer' to 'TB_BRAIN' table
pub fn q_brain_add_talk(question: String, answer: String) {
    security::q_security_add_security_timestamps(ADD_TALK);
    let path = get_file_path(format!("{}{}", db_folder(), DATABASE));
    let conn = sqlite::open(path).unwrap();

    let query = format!(
        "
    INSERT INTO {TB_BRAIN} ({CL_TIMESTAMP}, {CL_QUESTIONS}, {CL_ANSWERS}) 
    VALUES (datetime(CURRENT_TIMESTAMP,'localtime'), '{:?}', '{:?}' );",
        r#question, r#answer
    );

    println!("{:?}", r#answer);

    conn.execute(query)
        .expect("\x1b[0m\x1b[31;3mFailed to execute query! 'q_brain_add_talk()'\x1b[0m");
}

/// SEARCH BRAIN TALKS
pub fn q_brain_search_talks() {
    security::q_security_add_security_timestamps(SEARCH_TALK);
    views::start_menus("Search anything!");

    println!("\n{CYAN_UNDERLINE}Please insert the pattern you want to search:{CLOSE}");
    let search_keyword = get_user_input();

    let path = get_file_path(format!("{}{}", db_folder(), DATABASE));
    let conn = sqlite::open(path).unwrap();

    let query =
        format!("SELECT {CL_ID}, {CL_TIMESTAMP}, {CL_QUESTIONS}, {CL_ANSWERS} FROM {TB_BRAIN} WHERE {CL_QUESTIONS} LIKE '%{search_keyword}%' OR {CL_ANSWERS} LIKE '%{search_keyword}%';");
    let mut statement = conn.prepare(query).unwrap();

    statement.iter();

    println!("\n{BLUE}###############################################{CLOSE}\n");
    println!(
        "\n{CYAN_UNDERLINE}Table{CLOSE}   : {TB_BRAIN}\n{CYAN_UNDERLINE}Columns{CLOSE} : {CL_ID} | {CL_TIMESTAMP} | {CL_QUESTIONS} | {CL_ANSWERS}\n\n"
    );
    while let Ok(State::Row) = statement.next() {
        println!(
            "\n> {} | {} | {} | {}\n",
            statement.read::<String, _>(CL_ID).unwrap(),
            statement.read::<String, _>(CL_TIMESTAMP).unwrap(),
            statement.read::<String, _>(CL_QUESTIONS).unwrap(),
            statement.read::<String, _>(CL_ANSWERS).unwrap(),
        );
    }
    println!(
        "{BLUE}###############################################\nPress ENTER to continue...{CLOSE}"
    );
    get_user_input();
}

/// DELETE BRAIN TALKS
pub fn q_brain_delete_talk() {
    security::q_security_add_security_timestamps(REMOVE_TALK);
    views::start_menus("Delete your conversations!");

    let path = get_file_path(format!("{}{}", db_folder(), DATABASE));
    let conn = sqlite::open(path).unwrap();

    q_brain_show_all();
    println!("\n{CYAN_UNDERLINE}Please insert 'ID':\n{BLUE}(or press ENTER to continue){CLOSE}");
    let talk_id = get_user_input();

    if !talk_id.trim().is_empty() && talk_id.chars().next().unwrap().is_ascii_digit() {
        let query = format!(
            "
                DELETE FROM {TB_BRAIN} where {CL_ID} = {:?}; VACUUM;
                ",
            talk_id.trim().parse::<i32>().unwrap()
        );
        match conn.execute(query) {
            Ok(_) => {
                println!("{CYAN_UNDERLINE}Talk id '{talk_id}' was successfully deleted{CLOSE}")
            }
            Err(_) => println!("{ERRO}Failed to execute query! 'q_brain_add_talk()'{CLOSE}"),
        }
    }
}

/// SHOW ALL TALKS
pub fn q_brain_show_all() {
    security::q_security_add_security_timestamps(VIEW_TALK);
    views::start_menus("Brain History Log!");
    let path = get_file_path(format!("{}{}", db_folder(), DATABASE));
    let conn = sqlite::open(path).unwrap();
    let query =
        format!("SELECT {CL_ID}, {CL_TIMESTAMP}, {CL_QUESTIONS}, {CL_ANSWERS} FROM {TB_BRAIN};");
    let mut statement = conn.prepare(query).unwrap();

    statement.iter();

    println!(
        "\n{CYAN_UNDERLINE}Table{CLOSE}   : {TB_BRAIN}\n{CYAN_UNDERLINE}Columns{CLOSE} : {CL_ID} | {CL_TIMESTAMP} | {CL_QUESTIONS} | {CL_ANSWERS}\n"
    );
    while let Ok(State::Row) = statement.next() {
        println!(
            "\n> {} | {} | {} | {}\n",
            statement.read::<String, _>(CL_ID).unwrap(),
            statement.read::<String, _>(CL_TIMESTAMP).unwrap(),
            statement.read::<String, _>(CL_QUESTIONS).unwrap(),
            statement.read::<String, _>(CL_ANSWERS).unwrap(),
        );
    }
    println!("\n{BLUE}###############################################{CLOSE}\n");
}
