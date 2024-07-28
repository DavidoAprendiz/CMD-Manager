use std::env;
pub mod brain;
pub mod security;
pub mod todo;

// Database file name
const DATABASE: &str = "database.db";

//Tables
pub const TB_TODO: &str = "TODO";
const TB_SECURITY: &str = "SECURITY";
const TB_BRAIN: &str = "BRAIN";

// DB SECURITY_HIST columns
const CL_REASON: &str = "reason";

// DB TODO_HIST columns
pub const CL_TODO_TITLE: &str = "title";
pub const CL_TODO_TASK: &str = "task";

// DB BRAIN_HIST columns
const CL_QUESTIONS: &str = "questions";
const CL_ANSWERS: &str = "answers";

// Shared columns
pub const CL_ID: &str = "rowid";
pub const CL_TIMESTAMP: &str = "timestamp";

// SECURITY_HIST reasons
pub const SYSTEM_LOGON: &str = "system_startup";
pub const TODO_LOGON: &str = "todo_logon";
pub const FILE_LOGON: &str = "file_logon";
pub const WEB_LOGON: &str = "web_logon";
pub const BRAIN_LOGON: &str = "brain_logon";
pub const START_TALK: &str = "conversation_started";
pub const SAVE_MD: &str = "markdown_saved";
const ADD_TALK: &str = "conversation_saved";
const REMOVE_TALK: &str = "conversation_removed";
const SEARCH_TALK: &str = "conversation_searched";
const VIEW_TALK: &str = "conversations_log_opened";
const VIEW_SECURITY: &str = "security_log_opened";
const VIEW_TODO: &str = "todo_log_opened";
const ADD_TASK: &str = "task_added";
const REMOVE_TASK: &str = "task_removed";
const SEARCH_TODO: &str = "todo_searched";

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
