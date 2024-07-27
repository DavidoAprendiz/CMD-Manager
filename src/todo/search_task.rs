use crate::utils;

pub fn search_task() {
    utils::clear_screen();

    println!("\n... under-construction ...");

    println!(
        "\n{}###############################################\nPress ENTER to continue...{}",
        utils::BLUE,
        utils::CLOSE
    );
    utils::get_user_input();
}
