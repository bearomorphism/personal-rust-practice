use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");

    let secret_number = rand::rng().random_range(1..=100);

    #[allow(while_true)]
    while true {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
    println!("The secret number was: {}", secret_number);
}
