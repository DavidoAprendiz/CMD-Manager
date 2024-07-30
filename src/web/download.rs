use crate::{
    queries::{q_save_data, ADD_DOWNLOAD, CL_DOWNLOAD_BODY, CL_DOWNLOAD_URL, TB_WEB_DOWNLOAD},
    utils::{get_user_input, BLUE, CLOSE, CYAN_UNDERLINE, ERRO},
    views,
};
use reqwest::blocking::get;

/// Download
///
/// Check folder Downloads (if !exists and OS), get user input and write directly to file
pub fn main() {
    views::start_menus("Download data!");
    println!("{CYAN_UNDERLINE}Enter the url to download:{CLOSE}");
    let url = get_user_input();
    let url_response = get_response(url.clone());
    q_save_data(
        url,
        url_response,
        TB_WEB_DOWNLOAD,
        CL_DOWNLOAD_URL,
        CL_DOWNLOAD_BODY,
        ADD_DOWNLOAD,
    );
    println!("{BLUE}Press ENTER to continue...{CLOSE}");
    get_user_input();
}

/// Generate a response from an URL.
fn get_response(user_input: String) -> String {
    match get(user_input) {
        Ok(text) => text
            .text()
            .expect("\x1b[0m\x1b[31;3mFailed to display json.\x1b[0m"),
        Err(e) => format!("{ERRO}Failed to process request...\n{e}{CLOSE}\n"),
    }
}
