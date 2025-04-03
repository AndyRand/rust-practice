/*
Find the number game

This time it's my turn to guess the number!
You will choose a number between 1 and 100.
I will try to guess it in as few attempts as possible.
Every attempt, I will ask you if my guess is too high or too low.
You will respond with "too high", "too low", or "correct" by choosing one of the proposed options.

*/

fn main() {
    println!("Find the number game");
    println!("This time it's my turn to guess the number!");
    println!("You will choose a number between 1 and 100.");
    println!("I will try to guess it in as few attempts as possible.");
    println!(
        "You will respond with \"too high\", \"too low\", or \"correct\" by choosing one of the proposed options.",
    );

    let mut max = 100;
    let mut min = 0;
    let mut guess = 50;

    loop {
        println!("===========================");
        println!("Is your number {}?", guess);
        println!("Too high, too low, or correct?");
        println!("1: Too high");
        println!("2: Too low");
        println!("3: Correct");
        println!("");

        let mut response = String::new();
        std::io::stdin()
            .read_line(&mut response)
            .expect("Failed to read line");

        match response.trim() {
            "1" => {
                println!("Too high? let me try again");
                max = guess;
                guess = guess - (guess - min) / 2;
            }
            "2" => {
                println!("Too low? let me try again");
                min = guess;
                guess = guess + (max - guess) / 2;
            }
            "3" => {
                println!("Correct");
                break;
            }
            _ => println!("Invalid response"),
        }
    }
}
