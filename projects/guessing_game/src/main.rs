use std::io::stdin;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");
        // that mut makes guess mutable
        let mut guess = String::new();
        stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // Oh, ooh.  Some sort of String interpolation here
        println!("Your guess: {}", guess);

        // The group isn't so sure allowing this "shadow definition" here is really a good idea.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\"{}\" is not a number!  Try again.", guess.trim());
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
