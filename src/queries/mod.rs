use crate::utils;
use std::env;
pub mod brain;
pub mod file;
pub mod security;
pub mod todo;

// Database file name
pub const DATABASE: &str = "database.db";

// Tables
pub const TB_TODO: &str = "TODO";
const TB_SECURITY: &str = "SECURITY";
pub const TB_BRAIN: &str = "BRAIN";
pub const TB_WEB_API: &str = "API";
pub const TB_WEB_DOWNLOAD: &str = "DOWNLOAD";
pub const TB_FILE_COMPARE: &str = "FILE_COMPARE";
pub const TB_FILE_SEARCH: &str = "FILE_SEARCH";

// SECURITY columns
const CL_REASON: &str = "security_tag";

// .TODO columns
pub const CL_TODO_TITLE: &str = "title";
pub const CL_TODO_TASK: &str = "task";

// WEB columns
pub const CL_DOWNLOAD_URL: &str = "download_url";
pub const CL_DOWNLOAD_BODY: &str = "download_body";
pub const CL_WEB_NAME: &str = "name";
pub const CL_WEB_PRICE: &str = "price";

// FILE columns
pub const CL_FILE_PATTERN: &str = "file_pattern";
pub const CL_FILENAME_1: &str = "file_name_1";
pub const CL_FILENAME_2: &str = "file_name_2";
pub const CL_FILE_CONTENT: &str = "file_content";

// DB BRAIN columns
pub const CL_QUESTIONS: &str = "questions";
pub const CL_ANSWERS: &str = "answers";

// Shared columns
pub const CL_ID: &str = "rowid";
pub const CL_TIMESTAMP: &str = "timestamp";

// SECURITY reasons
pub const SYSTEM_LOGON: &str = "system_startup";
const VIEW_SECURITY: &str = "security_log_opened";
pub const TODO_LOGON: &str = "todo_logon";
const ADD_TASK: &str = "task_added";
const VIEW_TODO: &str = "todo_log_opened";
const REMOVE_TASK: &str = "task_removed";
const SEARCH_TODO: &str = "todo_searched";
pub const BRAIN_LOGON: &str = "brain_logon";
pub const START_TALK: &str = "conversation_started";
const ADD_TALK: &str = "conversation_saved";
const REMOVE_TALK: &str = "conversation_removed";
const SEARCH_TALK: &str = "conversation_searched";
const VIEW_TALK: &str = "conversations_log_opened";
pub const SAVE_MD: &str = "markdown_saved";
pub const WEB_LOGON: &str = "web_logon";
pub const ADD_DOWNLOAD: &str = "download_saved";
pub const ADD_PRICE: &str = "price_saved";
pub const FILE_LOGON: &str = "file_logon";
pub const SEARCH_FILE: &str = "file_searched";
pub const COMPARE_FILE: &str = "file_compared";

/// Create all tables (if not exists).
/// Add timestamp/logs to Table SECURITY
pub fn first_config() {
    utils::create_folder(db_folder());
    security::q_security_create_table();
    file::q_file_create_table(
        TB_FILE_SEARCH,
        CL_FILE_PATTERN,
        CL_FILENAME_1,
        CL_FILE_CONTENT,
    );
    file::q_file_create_table(
        TB_FILE_COMPARE,
        CL_FILENAME_1,
        CL_FILENAME_2,
        CL_FILE_CONTENT,
    );
    q_create_table(TB_TODO, CL_TODO_TITLE, CL_TODO_TASK);
    q_create_table(TB_BRAIN, CL_QUESTIONS, CL_ANSWERS);
    q_create_table(TB_WEB_API, CL_WEB_NAME, CL_WEB_PRICE);
    q_create_table(TB_WEB_DOWNLOAD, CL_DOWNLOAD_URL, CL_DOWNLOAD_BODY);
    security::q_security_add_timestamps(SYSTEM_LOGON);
}

/// Start DB Connection
pub fn start_db_connection() -> sqlite::Connection {
    let path = utils::get_file_path(format!("{}{}", db_folder(), DATABASE));
    sqlite::open(path).unwrap()
}

/// Create 'TB_...' tables
pub fn q_create_table(table: &str, col1: &str, col2: &str) {
    let conn = start_db_connection();
    let query = format!(
        "CREATE TABLE IF NOT EXISTS {table} ({CL_TIMESTAMP} TEXT, {col1} BLOB, {col2} BLOB);"
    );
    conn.execute(query)
        .expect("\x1b[0m\x1b[31;3mFailed to execute query! 'q_create_table()'\x1b[0m");
}

/// Save to database (2 columns)
pub fn q_save_data(
    name: String,
    url_body: String,
    table: &str,
    col1: &str,
    col2: &str,
    reason: &str,
) {
    security::q_security_add_timestamps(reason);
    let conn = start_db_connection();
    let query = format!(
        "
    INSERT INTO {table} ({CL_TIMESTAMP}, {col1}, {col2}) 
    VALUES (datetime(CURRENT_TIMESTAMP,'localtime'), '{:?}', '{:?}' );",
        r#name,
        r#url_body
            .replace('\'', "´")
            .replace('\n', "<br>")
            .replace('\"', "´"),
    );

    conn.execute(query)
        .expect("\x1b[0m\x1b[31;3mFailed to execute query! 'q_save_data()'\x1b[0m");
}

/// Create folder Database (if does't exist) and verify the running operating system (OS) and return the correct path.
pub fn db_folder() -> String {
    let operating_system = env::consts::OS;
    if operating_system.contains("windows") {
        "\\Project\\".to_string()
    } else {
        "/Project/".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _db_folder() {
        assert_eq!(
            db_folder(),
            if env::consts::OS.contains("windows") {
                "\\Project\\".to_string()
            } else {
                "/Project/".to_string()
            }
        );
    }
}
