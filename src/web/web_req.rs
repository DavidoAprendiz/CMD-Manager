use crate::{utils, views};
use reqwest::blocking::get;

pub fn main() {
    views::start_menu_webreq();
    println!("Enter your option: ");
    let user_input = utils::get_user_input();

    match user_input.trim() {
        "1" => get_price_data("ergo".to_string()),
        "2" => get_price_data("cardano".to_string()),
        _ => (),
    }
}

fn get_price_data(name: String) {
    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=eur",
        name
    );

    let url_response = get(url)
        .expect("Failed to process request.")
        .text()
        .expect("Failed to display json.");

    println!(
        "{} is currently @ {:.5}€",
        &name,
        &url_response
            .replace("{\"ergo\":{\"eur\":", "")
            .replace("{\"cardano\":{\"eur\":", "")
            .replace("}}", "")
    );
    utils::get_user_input(); //to wait
}
