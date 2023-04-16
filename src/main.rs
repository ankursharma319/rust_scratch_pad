use std::io;
use rand::Rng;
use std::cmp::Ordering;

pub mod garden;

fn demo(s:&String) -> &String {
    let s2 = String::from("hello");
    let _slice:&str = &s2[0..2];
    // println!("{}", slice);
    garden::pluck_vegtable();
    garden::vegatables::sweeten();
    // below is error because fruits is private module
    // garden::fruits::sweeten();
    // the below is error because its a private function
    // garden::corrupt_vegetable();
    return s;
    // let mut s = String::from("  hello  ");
    //
    // let r1 = &s;
    // let r2 = &s;
    //
    // // s.push_str("hello there");
    //
    // println!("{}, {}", r1, r2);
    // return s;
}

fn main() {
    println!("Hello, world, guess the number!");

    let secret_num = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {}", secret_num);
    if secret_num < 50 {
        println!("Hint, go low");
    } else {
        println!("Hint, go high");
    }
    println!("Type your guess");

    // loop forever, until the break statement
    loop {
        let mut guess: String = String::new();
        demo(&guess);
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
