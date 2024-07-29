use crate::{queries, queries::security, utils, views};
use sqlite::State;

///
/// BRAIN MANAGER
///

/// Create 'TB_BRAIN' table
pub fn q_brain_create_table() {
    let conn = queries::start_db_connection();
    let query = format!(
        "CREATE TABLE IF NOT EXISTS {} ({} TEXT, {} BLOB, {} BLOB);",
        queries::TB_BRAIN,
        queries::CL_TIMESTAMP,
        queries::CL_QUESTIONS,
        queries::CL_ANSWERS
    );
    conn.execute(query)
        .expect("\x1b[0m\x1b[31;3mFailed to execute query! 'q_brain_create_table()'\x1b[0m");
}

/// Insert data 'timestamp',user 'question' and user 'answer' to 'TB_BRAIN' table
pub fn q_brain_add_talk(question: String, answer: String) {
    security::q_security_add_security_timestamps(queries::ADD_TALK);
    let conn = queries::start_db_connection();
    let query = format!(
        "
    INSERT INTO {} ({}, {}, {}) 
    VALUES (datetime(CURRENT_TIMESTAMP,'localtime'), '{:?}', '{:?}' );",
        queries::TB_BRAIN,
        queries::CL_TIMESTAMP,
        queries::CL_QUESTIONS,
        queries::CL_ANSWERS,
        r#question,
        r#answer
    );
    println!("{:?}", r#answer);
    conn.execute(query)
        .expect("\x1b[0m\x1b[31;3mFailed to execute query! 'q_brain_add_talk()'\x1b[0m");
}

/// SEARCH BRAIN TALKS
pub fn q_brain_search_talks() {
    security::q_security_add_security_timestamps(queries::SEARCH_TALK);
    views::start_menus("Search anything!");
    println!(
        "\n{}Please insert the pattern you want to search:{}",
        utils::CYAN_UNDERLINE,
        utils::CLOSE
    );
    let search_keyword = utils::get_user_input();
    let conn = queries::start_db_connection();
    let query =
        format!("SELECT {}, {}, {}, {} FROM {} WHERE {} LIKE '%{search_keyword}%' OR {} LIKE '%{search_keyword}%';", queries::CL_ID, queries::CL_TIMESTAMP, queries::CL_QUESTIONS, queries::CL_ANSWERS, queries::TB_BRAIN, queries::CL_QUESTIONS, queries::CL_ANSWERS);
    let mut statement = conn.prepare(query).unwrap();
    statement.iter();
    println!(
        "\n{}###############################################{}\n",
        utils::BLUE,
        utils::CLOSE
    );
    println!(
        "\n{}Table{}   : {}\n{}Columns{} : {} | {} | {} | {}\n\n",
        utils::CYAN_UNDERLINE,
        utils::CLOSE,
        queries::TB_BRAIN,
        utils::CYAN_UNDERLINE,
        utils::CLOSE,
        queries::CL_ID,
        queries::CL_TIMESTAMP,
        queries::CL_QUESTIONS,
        queries::CL_ANSWERS
    );
    while let Ok(State::Row) = statement.next() {
        println!(
            "\n> {} | {} | {} | {}\n",
            statement.read::<String, _>(queries::CL_ID).unwrap(),
            statement.read::<String, _>(queries::CL_TIMESTAMP).unwrap(),
            statement.read::<String, _>(queries::CL_QUESTIONS).unwrap(),
            statement.read::<String, _>(queries::CL_ANSWERS).unwrap(),
        );
    }
    println!(
        "{}###############################################\nPress ENTER to continue...{}",
        utils::BLUE,
        utils::CLOSE
    );
    utils::get_user_input();
}

/// DELETE BRAIN TALKS
pub fn q_brain_delete_talk() {
    security::q_security_add_security_timestamps(queries::REMOVE_TALK);
    views::start_menus("Delete your conversations!");
    let conn = queries::start_db_connection();
    q_brain_show_all();
    println!(
        "\n{}Please insert 'ID':\n{}(or press ENTER to continue){}",
        utils::CYAN_UNDERLINE,
        utils::BLUE,
        utils::CLOSE
    );
    let talk_id = utils::get_user_input();

    if !talk_id.trim().is_empty() && talk_id.chars().next().unwrap().is_ascii_digit() {
        let query = format!(
            "
                DELETE FROM {} where {} = {:?}; VACUUM;
                ",
            queries::TB_BRAIN,
            queries::CL_ID,
            talk_id.trim().parse::<i32>().unwrap()
        );
        match conn.execute(query) {
            Ok(_) => {
                println!(
                    "{}Talk id '{talk_id}' was successfully deleted{}",
                    utils::CYAN_UNDERLINE,
                    utils::CLOSE
                )
            }
            Err(_) => println!(
                "{}Failed to execute query! 'q_brain_add_talk()'{}",
                utils::ERRO,
                utils::CLOSE
            ),
        }
    }
}

/// SHOW ALL TALKS
pub fn q_brain_show_all() {
    security::q_security_add_security_timestamps(queries::VIEW_TALK);
    views::start_menus("Brain History Log!");
    let conn = queries::start_db_connection();
    let query = format!(
        "SELECT {}, {}, {}, {} FROM {};",
        queries::CL_ID,
        queries::CL_TIMESTAMP,
        queries::CL_QUESTIONS,
        queries::CL_ANSWERS,
        queries::TB_BRAIN
    );
    let mut statement = conn.prepare(query).unwrap();
    statement.iter();
    println!(
        "\n{}Table{}   : {}\n{}Columns{} : {} | {} | {} | {}\n",
        utils::CYAN_UNDERLINE,
        utils::CLOSE,
        queries::TB_BRAIN,
        utils::CYAN_UNDERLINE,
        utils::CLOSE,
        queries::CL_ID,
        queries::CL_TIMESTAMP,
        queries::CL_QUESTIONS,
        queries::CL_ANSWERS
    );
    while let Ok(State::Row) = statement.next() {
        println!(
            "\n> {} | {} | {} | {}\n",
            statement.read::<String, _>(queries::CL_ID).unwrap(),
            statement.read::<String, _>(queries::CL_TIMESTAMP).unwrap(),
            statement.read::<String, _>(queries::CL_QUESTIONS).unwrap(),
            statement.read::<String, _>(queries::CL_ANSWERS).unwrap(),
        );
    }
    println!(
        "\n{}###############################################{}\n",
        utils::BLUE,
        utils::CLOSE
    );
}
