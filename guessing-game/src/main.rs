// Creating a guessing game in which a user will enter a guess and will keep on playing untill the guess is correct.
use std::io ;
use std::cmp::Ordering ;
use rand::Rng ;

fn main() {
    println!("Guess the number !!") ;
    // Creating a random number in the range of 1 and 20.
    let secret_num = rand::thread_rng().gen_range(1..21) ;

    loop {
        println!("Please enter your guess:");
        // Creating an empty string to take input from the user
        let mut guess = String::new() ;

        // Calling stdin() fn that represents a handle to the standard input for your terminal.
        io::stdin()
        .read_line(&mut guess)      // To get input from the user 
        .expect("Failed in reading input!!")  ;          // Handling a potential failure in reading input

        let guess: u32 = match guess.trim().parse() {       // Handling parsing error with "Result" 
            Ok(num) => num,                     
            Err(_) => continue
        } ;

        if guess > 20 {
            println!("Please enter a guess between the range 1 and 20 !!") ;
            continue ;
        }

        // Matching user's input with the secret number using a match expression
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Number entered is smaller !!"),
            Ordering::Greater => println!("Number entered is greater !!") ,
            Ordering::Equal => {
                println!("You guessed correctly. You WIN !!") ;
                break ;
            }
        }
    }
}