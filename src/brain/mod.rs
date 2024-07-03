use crate::{utils, views};
use sqlite::State;
use std::env;

const DB_NAME: &str = "LOGIN_HIST";
const LAST_LOGIN: &str = "last_login";

pub fn main() {
    utils::create_folder(db_folder());
    // Create DB, if doesn't exist
    query_create_db(DB_NAME);

    // Add "last-login" timestamp
    query_add_timestamp(DB_NAME);
    views::start_menu_db();
    'main_loop: loop {
        println!("Enter your option: ");
        let user_input = utils::get_user_input();
        match user_input.trim() {
            "1" => query_add_timestamp(DB_NAME),
            "2" => query_show_all(DB_NAME),
            _ => {
                if utils::exit_program(&user_input) {
                    break 'main_loop;
                }
            }
        }
        views::start_menu_db();
    }
    utils::clear_screen();
}

fn db_folder() -> String {
    let operating_system = env::consts::OS;
    if operating_system.contains("windows") {
        "\\Project\\Database\\".to_string()
    } else {
        "/Project/Database/".to_string()
    }
}

fn query_create_db(db_name: &str) {
    let path = utils::get_file_path(format!("{}database.db", db_folder()));
    let conn = sqlite::open(path).unwrap();

    let query = format!("CREATE TABLE {db_name} ({LAST_LOGIN} TEXT);");

    match conn.execute(query) {
        Ok(_) => (),
        Err(e) => println!("\nDatabase table already created.\n{e}\n"),
    };
}

fn query_add_timestamp(db_name: &str) {
    let path = utils::get_file_path(format!("{}database.db", db_folder()));
    let conn = sqlite::open(path).unwrap();
    let query = format!(
        "
    INSERT INTO {db_name} ({LAST_LOGIN}) 
    VALUES (datetime(CURRENT_TIMESTAMP, 'localtime'));
    "
    );
    match conn.execute(query) {
        Ok(_) => {
            utils::clear_screen();
            println!("Timestamp added!");
            println!("Press ENTER to continue...");
            utils::get_user_input();
        }
        Err(e) => println!("\n{e}\n"),
    };
}

fn query_show_all(db_name: &str) {
    let path = utils::get_file_path(format!("{}database.db", db_folder()));
    let conn = sqlite::open(path).unwrap();
    let query = format!("SELECT * FROM {db_name};");
    let mut statement = conn.prepare(query).unwrap();
    statement.iter();

    println!("\n########################");
    println!("TABLE: {DB_NAME}\n Column: {LAST_LOGIN} History\n");
    while let Ok(State::Row) = statement.next() {
        println!(
            "timestamp = {}",
            statement.read::<String, _>(LAST_LOGIN).unwrap(),
        );
    }
    println!("\n########################\n");
    println!("Press ENTER to continue...");
    utils::get_user_input();
}
