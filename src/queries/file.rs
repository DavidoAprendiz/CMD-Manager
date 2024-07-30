use crate::queries::{
    start_db_connection, CL_FILENAME, CL_FILENAME_2, CL_FILE_DIFF, CL_TIMESTAMP, TB_FILE_COMPARE,
};

///
/// FILE MANAGER
///

/// Create 'TB_FILE_COMPARE' table
pub fn q_file_compare_create_table() {
    let conn = start_db_connection();
    let query = format!(
        "CREATE TABLE IF NOT EXISTS {TB_FILE_COMPARE} ({CL_TIMESTAMP} TEXT, {CL_FILENAME} TEXT,{CL_FILENAME_2} TEXT, {CL_FILE_DIFF} BLOB);"
    );
    conn.execute(query)
        .expect("\x1b[0m\x1b[31;3mFailed to execute query! 'q_file_compare_create_table()'\x1b[0m");
}
