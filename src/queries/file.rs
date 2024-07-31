use crate::queries::{security, start_db_connection, CL_FILE_CONTENT, CL_TIMESTAMP};

///
/// FILE MANAGER
///

/// Create 'TB_FILE_*' tables
pub fn q_file_create_table(table: &str, col1: &str, col2: &str, col3: &str) {
    let conn = start_db_connection();
    let query = format!(
        "CREATE TABLE IF NOT EXISTS {table} ({CL_TIMESTAMP} TEXT, {col1} TEXT, {col2} TEXT, {col3} BLOB);"
    );
    conn.execute(query)
        .expect("\x1b[0m\x1b[31;3mFailed to execute query! 'q_file_compare_create_table()'\x1b[0m");
}

/// Save to database (3 columns)
pub fn q_file_save_data(
    table: &str,
    col1: &str,
    col2: &str,
    pattern_or_filename: &str,
    filename2: &str,
    output: &str,
    reason: &str,
) {
    security::q_security_add_timestamps(reason);
    let conn = start_db_connection();
    let query = format!(
        "
    INSERT INTO {table} ({CL_TIMESTAMP}, {col1}, {col2}, {CL_FILE_CONTENT}) 
    VALUES (datetime(CURRENT_TIMESTAMP,'localtime'), '{:?}', '{:?}', '{:?}');",
        r#pattern_or_filename,
        r#filename2,
        r#output
            .replace('\'', "´")
            .replace('\n', "<br>")
            .replace('\"', "´"),
    );

    conn.execute(query)
        .expect("\x1b[0m\x1b[31;3mFailed to execute query! 'q_save_data()'\x1b[0m");
}
