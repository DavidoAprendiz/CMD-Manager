use crate::utils;
mod web_req;

pub fn main() {
    utils::clear_screen();
    println!("Web Manager... (in-progress)");

    web_req::main();

    utils::get_user_input();
}
