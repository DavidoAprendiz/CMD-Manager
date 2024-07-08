use crate::{brain, brain::db_folder, utils, views};
use sqlite::State;
use std::fs;
use std::io::Write;
use std::process::Command;

// Constant for DB name
const DATABASE: &str = "database.db";

// Constants for tables
const TB_SECURITY: &str = "SECURITY_HIST";
const TB_BRAIN: &str = "BRAIN_HIST";

// Constants for shared columns
const CL_TIMESTAMP: &str = "timestamp";
// Constants for SECURITY_HIST columns
const CL_REASON: &str = "reasons";
pub const SYSTEM_LOGON: &str = "system_logon";
pub const TODO_LOGON: &str = "todo_logon";
pub const FILE_LOGON: &str = "file_logon";
pub const WEB_LOGON: &str = "web_logon";
pub const BRAIN_LOGON: &str = "brain_logon";
// Constants for BRAIN columns
const CL_QUESTIONS: &str = "questions";
const CL_ANSWERS: &str = "answers";

///
/// START TALK
///

/// Start a new conversation
pub fn start_new_talk() {
    views::start_menu_brain_new_talk();
    let user_input = utils::get_user_input();

    println!("\n... waiting response ...\n");

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
    .replace('\n', " <br>")
    .replace('\"', "´");

    q_brain_add_talk(user_input.to_owned(), comm.to_owned());

    println!("\nSave to Markdown file?\n(Write 'Yes' or press ENTER to continue...)");

    let save = utils::get_user_input();
    if save.to_lowercase().starts_with('y') {
        talk_save_to_md(
            user_input
                .replace('\'', "´")
                .replace('\n', " <br>")
                .replace('\"', "´"),
            comm,
        );
        println!("Press ENTER to continue...");
        utils::get_user_input();
    }
}

/// Save conversation to a Markdown file
fn talk_save_to_md(user_input: String, comm: String) {
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
        Ok(_) => println!("File saved successfully."),
        Err(e) => println!("\x1b[0m\x1b[31;3mFailed to save task in file!\n{e}\x1b[0m\n"),
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
        .expect("Failed to execute query! 'q_security_create_table()'\n");
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
        .expect("Failed to execute query! 'q_security_add_timestamp()'\n");
}

/// DELETE SECURITY DATA ?

/// Show all data from 'TB_SECURITY' table
pub fn q_security_show_all() {
    let path = utils::get_file_path(format!("{}{}", db_folder(), DATABASE));
    let conn = sqlite::open(path).unwrap();
    let query = format!("SELECT * FROM {TB_SECURITY};");
    let mut statement = conn.prepare(query).unwrap();
    statement.iter();

    println!("\n########################");
    println!("TABLE: {TB_SECURITY}\n Columns: {CL_TIMESTAMP} | {CL_REASON}\n");
    while let Ok(State::Row) = statement.next() {
        println!(
            "> {} | {}",
            statement.read::<String, _>(CL_TIMESTAMP).unwrap(),
            statement.read::<String, _>(CL_REASON).unwrap(),
        );
    }
    println!("\n########################\n");
    println!("Press ENTER to continue...");
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
        "CREATE TABLE IF NOT EXISTS {TB_BRAIN} (timestamp TEXT, questions BLOB, answers BLOB);"
    );

    conn.execute(query)
        .expect("Failed to execute query! 'q_brain_create_table()'");
}

/// Insert data 'timestamp',user 'question' and user 'answer' to 'TB_BRAIN' table
pub fn q_brain_add_talk(question: String, answer: String) {
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
        .expect("Failed to execute query! 'q_brain_add_talk()'");
}

// DELETE BRAIN TALKS

// SHOW ALL TALKS
