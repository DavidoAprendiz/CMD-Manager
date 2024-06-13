pub fn main() {
    guessing_game()
}
/// You have to think 'outside' the box :)
fn guessing_game() {
    println!("\nLets play a game...");
    println!("Pick a number from 1 to 100:");

    let number: i32 = user_inputs();

    if !(1..=100).contains(&number) {
        println!(
            "\nNumber: {}.You broke the system.Congratulations!!!\n", // ;)
            number
        )
    } else {
        println!("Number: {}. You failed. Try again!\n", number)
    }
}

fn user_inputs() -> i32 {
    let mut user_input: String = String::new();
    let resultado = std::io::stdin().read_line(&mut user_input);

    match resultado {
        Ok(_) => user_input.trim().parse::<i32>().unwrap(),
        Err(_) => {
            println!("failed to read line.please try again.");
            user_inputs()
        }
    }
}
