use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello, world, guess the number!");

    let secret_num = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {}", secret_num);

    println!("Type your guess");
    loop {
        let mut guess: String = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read from stdin");

        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Your guess is too low"),
            Ordering::Greater => println!("Your guess is too high"),
            Ordering::Equal => {
                println!("Your guess is correct! You win!");
                break;
            },
        }
    }
}
