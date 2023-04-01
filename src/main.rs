use std::io;
use rand::Rng;

fn main() {
    println!("Guess the sequence!");
    let mut guess = String::new();
    let mut rng = rand::thread_rng();
    let first_number = rng.gen_range(1..5);
    let last_number = rng.gen_range(1..5);
    let mut sequence: Vec<i32> = Vec::new();

    // create a sequence of numbers that are a multiple of the first and last numbers
    for i in 1..=4 {
        sequence.push(first_number * i);
    }
    sequence.push(last_number * 5);

    let num_guesses = sequence.len();
    let mut guesses_left = num_guesses;

    loop {
        println!("You have {} guesses left.", guesses_left);
        println!("Enter your guess (comma separated numbers):");
        guess.clear();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: Vec<i32> = guess.trim().split(',').map(|s| s.trim().parse().unwrap()).collect();
        guesses_left -= 1;
        if guess == sequence {
            println!("Congratulations! You guessed the sequence correctly.");
            break;
        } else {
            if guesses_left == 0 {
                println!("Sorry, you are out of guesses.");
                println!("The correct sequence was: {:?}", sequence);
                break;
            } else {
                println!("Incorrect guess. Please try again.");
            }
        }
    }
}
