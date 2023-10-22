use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    loop {
        println!("Guess the number!");

        println!("Please input your guess.");

        let secretNumber = rand::thread_rng().gen_range(1..=10);
        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse()
        {
            Ok(num) => num,
            Err(_) => { println!("Error, invalid number!");
                continue;},
        };

        println!("You guessed: {guess} \n");

        match guess.cmp(&secretNumber)
        {
            Ordering::Less => println!("Too Small"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            },
            Ordering::Greater => println!("Too Big!"),
        }
    }
}