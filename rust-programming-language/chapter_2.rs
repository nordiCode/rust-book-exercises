use std::cmp::Ordering;
use std::io;

fn main() {
    println!("***Guess the number***");
    println!("Enter a number? (1-10)");

    let secret_num: u32 = 5;

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Enter a number:");

        println!("You guess: {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
            Ordering::Greater => println!("Too Big!"),
        }
    }
}
