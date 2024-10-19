use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1..100);

    'game: loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("invalid input");
                continue;
            }
        };

        match guess {
            x if x > secret_number => {
                if (x - 10) > secret_number {
                    println!("too big")
                } else {
                    println!("close")
                }
            }
            x if x < secret_number => {
                if (x + 10) < secret_number {
                    println!("too small")
                } else {
                    println!("close")
                }
            }
            _ => {
                println!("correct. You Win!");
                break 'game;
            }
        }
    }
}
