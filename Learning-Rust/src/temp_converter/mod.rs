pub fn main() {
    convert_temperature()
}

fn convert_temperature() {
    println!("\nInput temperature value:");
    let temperature_number: String = user_inputs();

    println!("Input symbol: ");
    let temperature_symbol: String = user_inputs();

    match temperature_symbol.trim().parse::<char>() {
        Ok('c') => println!(
            "\n{} Fahrenheit -> {} Celsius.\n",
            temperature_number.trim(),
            output_convert(
                temperature_number.trim().parse::<f32>().unwrap(),
                temperature_symbol.trim().parse::<char>().unwrap()
            )
        ),
        Ok('f') => println!(
            "\n{} Celsius -> {} Fahrenheit.\n",
            temperature_number.trim(),
            output_convert(
                temperature_number.trim().parse::<f32>().unwrap(),
                temperature_symbol.trim().parse::<char>().unwrap()
            )
        ),
        Err(_) => println!("Invalid...\n"),
        _ => println!("Invalid...\n"),
    }
}

fn output_convert(temperature_number: f32, temperature_symbol: char) -> f32 {
    match temperature_symbol {
        'c' => (temperature_number - 32.0) * (5.0 / 9.0),
        'f' => temperature_number * (9.0 / 5.0) + 32.0,
        _ => 0.0,
    }
}

fn user_inputs() -> String {
    let mut user_input: String = String::new();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("failed to read user input");
    user_input
}
