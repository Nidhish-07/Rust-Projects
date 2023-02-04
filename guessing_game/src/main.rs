use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number: {}", secret_number);

    loop {
        println!("Enter your input");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading input");

        println!("Your guess: {}", guess);

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_)=>continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {break println!("You win!")},
            Ordering::Greater => println!("Too big"),
            Ordering::Less => println!("Too small"),
        }
    }
}
