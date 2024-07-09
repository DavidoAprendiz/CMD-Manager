use crate::{brain, brain::db_folder, utils, views};
use sqlite::State;
use std::fs;
use std::io::Write;
use std::process::Command;

// Define constants for colors (change colors in utils.rs)
const CLOSE: &str = utils::CLOSE;
const BLUE: &str = utils::BLUE;
const CYAN_UNDERLINE: &str = utils::CYAN_UNDERLINE;
const CYAN_UNDERLINE_BOLD: &str = utils::CYAN_UNDERLINE_BOLD;
const ERRO: &str = utils::ERRO;

//
// Constants
//
// Database file name
const DATABASE: &str = "database.db";
// Tables
const TB_SECURITY: &str = "SECURITY_HIST";
const TB_BRAIN: &str = "BRAIN_HIST";
// Shared columns
const CL_ID: &str = "rowid";
const CL_TIMESTAMP: &str = "timestamp";
// DB SECURITY_HIST columns
const CL_REASON: &str = "reasons";
// DB BRAIN_HIST columns
const CL_QUESTIONS: &str = "questions";
const CL_ANSWERS: &str = "answers";
// SECURITY_HIST reasons
pub const SYSTEM_LOGON: &str = "system_logon";
pub const TODO_LOGON: &str = "todo_logon";
pub const FILE_LOGON: &str = "file_logon";
pub const WEB_LOGON: &str = "web_logon";
pub const BRAIN_LOGON: &str = "brain_logon";
const START_TALK: &str = "conversation_started";
const ADD_TALK: &str = "conversation_saved";
const REMOVE_TALK: &str = "conversation_removed";
const SEARCH_TALK: &str = "conversation_searched";
const VIEW_TALK: &str = "conversations_log_opened";
const VIEW_SECURITY: &str = "security_log_opened";
const SAVE_MD: &str = "markdown_saved";

///
/// START TALK
///

/// Start a new conversation
pub fn start_new_talk() {
    q_security_add_security_timestamps(START_TALK);
    views::start_menus("Let's brainstorm!");
    println!("\n{CYAN_UNDERLINE}What's your question?{CLOSE}");
    let user_input = utils::get_user_input();

    println!("\n{CYAN_UNDERLINE_BOLD}... waiting response ...{CLOSE}\n");

    let comm = String::from_utf8(
        Command::new("/usr/local/bin/ollama")
            .args(["run", "llama3"])
            .arg(&user_input)
            .output()
            .expect("msg")
            .stdout,
    )
    .ok()
    .unwrap()
    .replace('\'', "´")
    .replace('\n', "<br>")
    .replace('\"', "´");

    q_brain_add_talk(
        user_input.replace("<br>", "").to_owned(),
        comm.replace("<br>", "").to_owned(),
    );

    println!("\n{CYAN_UNDERLINE}Save to Markdown file?\n{CLOSE}{BLUE}(write 'Yes' or press ENTER to continue...{CLOSE})");

    let save = utils::get_user_input();
    if save.to_lowercase().starts_with('y') {
        talk_save_to_md(
            user_input
                .replace('\'', "´")
                .replace('\n', "<br>")
                .replace('\"', "´"),
            comm,
        );
        println!("{BLUE}Press ENTER to continue...{CLOSE}");
        utils::get_user_input();
    }
}

/// Save conversation to a Markdown file
fn talk_save_to_md(user_input: String, comm: String) {
    q_security_add_security_timestamps(SAVE_MD);
    let path: String = {
        format!(
            "{}{}{}.md",
            std::env::current_dir()
                .expect("\x1b[0m\x1b[31;3mFailed to access current directory.\x1b[0m\n")
                .display(),
            brain::db_folder(),
            utils::get_current_time()
        )
    };

    let mut file =
        fs::File::create(path).expect("\x1b[0m\x1b[31;3mFailed to create text file!\x1b[0m");
    match file.write_fmt(format_args!(
        "\nQuestion: {}\n\nAnswer:{:?}\n",
        user_input, comm,
    )) {
        Ok(_) => println!("\n{CYAN_UNDERLINE}File saved successfully.{CLOSE}"),
        Err(e) => println!("{ERRO}Failed to save task in file!\n{e}{ERRO}"),
    }
}

///
/// SECURITY MANAGER
///

/// Create 'TB_SECURITY' table
pub fn q_security_create_table() {
    utils::create_folder(db_folder());
    let path = utils::get_file_path(format!("{}{}", db_folder(), DATABASE));
    let conn = sqlite::open(path).unwrap();

    let query = format!(
        "CREATE TABLE IF NOT EXISTS {TB_SECURITY} ({CL_TIMESTAMP} TEXT, {CL_REASON} TEXT);"
    );

    conn.execute(query)
        .expect("\x1b[0m\x1b[31;3mFailed to execute query! 'q_security_create_table()'\x1b[0m\n");
}

/// Insert 'reason'  to 'TB_SECURITY' table.
pub fn q_security_add_security_timestamps(reason: &str) {
    let path = utils::get_file_path(format!("{}{}", db_folder(), DATABASE));
    let conn = sqlite::open(path).unwrap();
    let query = format!(
        "
    INSERT INTO {TB_SECURITY} ({CL_TIMESTAMP}, {CL_REASON}) 
    VALUES (datetime(CURRENT_TIMESTAMP,'localtime'), '{reason}');
    "
    );
    conn.execute(query)
        .expect("\x1b[0m\x1b[31;3mFailed to execute query! 'q_security_add_timestamp()'\x1b[0m\n");
}

/// DELETE SECURITY DATA ?

/// Show all data from 'TB_SECURITY' table
pub fn q_security_show_all() {
    q_security_add_security_timestamps(VIEW_SECURITY);
    views::start_menus("Security History Log!");
    let path = utils::get_file_path(format!("{}{}", db_folder(), DATABASE));
    let conn = sqlite::open(path).unwrap();
    let query = format!("SELECT {CL_ID}, {CL_TIMESTAMP}, {CL_REASON} FROM {TB_SECURITY};");
    let mut statement = conn.prepare(query).unwrap();
    statement.iter();

    println!("\n{CYAN_UNDERLINE}Table{CLOSE} : {TB_SECURITY}\n    {CYAN_UNDERLINE}Columns{CLOSE} : {CL_ID} | {CL_TIMESTAMP} | {CL_REASON}\n");
    while let Ok(State::Row) = statement.next() {
        println!(
            "\n> {} | {} | {}",
            statement.read::<String, _>(CL_ID).unwrap(),
            statement.read::<String, _>(CL_TIMESTAMP).unwrap(),
            statement.read::<String, _>(CL_REASON).unwrap(),
        );
    }
    println!("\n{BLUE}###############################################\nPress ENTER to continue...{CLOSE}");
    utils::get_user_input();
}

///
/// BRAIN MANAGER
///

/// Create 'TB_BRAIN' table
pub fn q_brain_create_table() {
    utils::create_folder(db_folder());
    let path = utils::get_file_path(format!("{}{}", db_folder(), DATABASE));
    let conn = sqlite::open(path).unwrap();

    let query = format!(
        "CREATE TABLE IF NOT EXISTS {TB_BRAIN} ({CL_TIMESTAMP} TEXT, {CL_QUESTIONS} BLOB, {CL_ANSWERS} BLOB);"
    );

    conn.execute(query)
        .expect("\x1b[0m\x1b[31;3mFailed to execute query! 'q_brain_create_table()'\x1b[0m");
}

/// Insert data 'timestamp',user 'question' and user 'answer' to 'TB_BRAIN' table
pub fn q_brain_add_talk(question: String, answer: String) {
    q_security_add_security_timestamps(ADD_TALK);
    let path = utils::get_file_path(format!("{}{}", db_folder(), DATABASE));
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
    q_security_add_security_timestamps(SEARCH_TALK);
    views::start_menus("Search anything!");

    println!("\n{CYAN_UNDERLINE}Please insert the pattern you want to search:{CLOSE}");
    let search_keyword = utils::get_user_input();

    let path = utils::get_file_path(format!("{}{}", db_folder(), DATABASE));
    let conn = sqlite::open(path).unwrap();

    let query =
        format!("SELECT {CL_ID}, {CL_TIMESTAMP}, {CL_QUESTIONS}, {CL_ANSWERS} FROM {TB_BRAIN} WHERE {CL_QUESTIONS} LIKE '%{search_keyword}%' OR {CL_ANSWERS} LIKE '%{search_keyword}%';");
    let mut statement = conn.prepare(query).unwrap();

    statement.iter();

    println!("\n{BLUE}###############################################{CLOSE}\n");
    println!(
        "\n{CYAN_UNDERLINE}Table{CLOSE} : {TB_BRAIN}\n    {CYAN_UNDERLINE}Columns{CLOSE} : {CL_ID} | {CL_TIMESTAMP} | {CL_QUESTIONS} | {CL_ANSWERS}\n\n"
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
    utils::get_user_input();
}

/// DELETE BRAIN TALKS
pub fn q_brain_delete_talk() {
    q_security_add_security_timestamps(REMOVE_TALK);
    views::start_menus("Delete your conversations!");

    let path = utils::get_file_path(format!("{}{}", db_folder(), DATABASE));
    let conn = sqlite::open(path).unwrap();

    q_brain_show_all();
    println!("\n{CYAN_UNDERLINE}Please insert 'ID':\n{BLUE}(or press ENTER to continue){CLOSE}");
    let task_id = utils::get_user_input();

    if !task_id.trim().is_empty() && task_id.chars().next().unwrap().is_ascii_digit() {
        let query = format!(
            "
                DELETE FROM {TB_BRAIN} where {CL_ID} = {:?}; VACUUM;
                ",
            task_id.trim().parse::<i32>().unwrap()
        );
        match conn.execute(query) {
            Ok(_) => {
                println!("{CYAN_UNDERLINE}Task id '{task_id}' was successfully deleted{CLOSE}")
            }
            Err(_) => println!("{ERRO}TFailed to execute query! 'q_brain_add_talk()'{CLOSE}"),
        }
    }
}

/// SHOW ALL TALKS
pub fn q_brain_show_all() {
    q_security_add_security_timestamps(VIEW_TALK);
    views::start_menus("Brain History Log!");
    let path = utils::get_file_path(format!("{}{}", db_folder(), DATABASE));
    let conn = sqlite::open(path).unwrap();
    let query =
        format!("SELECT {CL_ID}, {CL_TIMESTAMP}, {CL_QUESTIONS}, {CL_ANSWERS} FROM {TB_BRAIN};");
    let mut statement = conn.prepare(query).unwrap();

    statement.iter();

    println!(
        "\n{CYAN_UNDERLINE}Table{CLOSE} : {TB_BRAIN}\n    {CYAN_UNDERLINE}Columns{CLOSE} : {CL_ID} | {CL_TIMESTAMP} | {CL_QUESTIONS} | {CL_ANSWERS}\n"
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
