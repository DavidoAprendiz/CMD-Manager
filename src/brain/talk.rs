use crate::{queries, utils::*, views};
use std::fs;
use std::io::Write;
use std::process::Command;

///
/// START TALK
///

/// Start a new conversation
pub fn start_new_talk() {
    queries::security::q_security_add_security_timestamps(queries::START_TALK);
    views::start_menus("Let's brainstorm!");
    println!("\n{CYAN_UNDERLINE}What's your question?{CLOSE}");
    let user_input = get_user_input();

    println!("\n{CYAN_UNDERLINE_BOLD}... waiting response ...{CLOSE}\n");

    let comm = String::from_utf8(
        Command::new("/usr/local/bin/ollama")
            .args(["run", "llama3.1"])
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

    queries::brain::q_brain_add_talk(
        user_input.replace("<br>", "").to_owned(),
        comm.replace("<br>", "").to_owned(),
    );

    println!("\n{CYAN_UNDERLINE}Save to Markdown file?\n{CLOSE}{BLUE}(write 'Yes' or press ENTER to continue...{CLOSE})");

    let save = get_user_input();
    if save.to_lowercase().starts_with('y') {
        talk_save_to_md(
            user_input
                .replace('\'', "´")
                .replace('\n', "<br>")
                .replace('\"', "´"),
            comm,
        );
        println!("{BLUE}Press ENTER to continue...{CLOSE}");
        get_user_input();
    }
}

/// Save conversation to a Markdown file
fn talk_save_to_md(user_input: String, comm: String) {
    queries::security::q_security_add_security_timestamps(queries::SAVE_MD);
    let path: String = {
        format!(
            "{}{}{}.md",
            std::env::current_dir()
                .expect("\x1b[0m\x1b[31;3mFailed to access current directory.\x1b[0m\n")
                .display(),
            queries::db_folder(),
            get_current_time()
        )
    };

    let mut file =
        fs::File::create(path).expect("\x1b[0m\x1b[31;3mFailed to create text file!\x1b[0m");
    match file.write_fmt(format_args!(
        "\nQuestion: {}\n\nAnswer:{:?}\n",
        user_input, comm,
    )) {
        Ok(_) => println!("\n{CYAN_UNDERLINE}File saved successfully.{CLOSE}"),
        Err(e) => println!("{ERRO}Failed to save talk in file!\n{e}{ERRO}"),
    }
}
