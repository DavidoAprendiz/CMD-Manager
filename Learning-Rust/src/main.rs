mod guess_game;
mod temp_converter;

fn main() {
    println!("\nHello everyone!\n");

    // Symbols Available: 'c' or 'f'
    temp_converter::main();

    // Guess a number from 1 to 100
    guess_game::main();
}
