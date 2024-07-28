use crate::{utils::*, views};
use reqwest::blocking::get;

/// Web Request
///
/// Start menu layout, ask user input, run API (if applicable)
pub fn main() {
    'main_loop: loop {
        views::start_menu_web_request();
        println!("Enter your option: ");
        let user_input = get_user_input();

        match user_input.trim() {
            "1" => get_price_data("ergo".to_string()),
            "2" => get_price_data("cardano".to_string()),
            _ => {
                if exit_program(&user_input) {
                    break 'main_loop;
                }
            }
        }
    }
}

/// Get the price data from CoinGecko API.
fn get_price_data(name: String) {
    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=eur",
        name
    );

    let url_response = get(url)
        .expect("\x1b[0m\x1b[31;3mFailed to process request.\x1b[0m")
        .text()
        .expect("\x1b[0m\x1b[31;3mFailed to display json.\x1b[0m");

    println!(
        "\n{CYAN_UNDERLINE_BOLD}{} is currently @ {:.5}€{CLOSE}",
        &name.to_uppercase(),
        &url_response
            .replace("{\"ergo\":{\"eur\":", "")
            .replace("{\"cardano\":{\"eur\":", "")
            .replace("}}", "")
    );
    println!("{BLUE}Press ENTER to exit...{CLOSE}");
    get_user_input();
}
