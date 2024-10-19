use std::{cmp::Ordering, io};
use rand::Rng;


fn main(){
    let x = 8;
    let y = 8;
    let res = x.cmp(&y).then_with(|| (x+4).cmp(&8));

    println!("{res:?}");
}

fn main1() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1..100);
    let mut correct_ans: bool = false;

    'game: loop {
        while !correct_ans {
            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            let guessed_number = guess.trim().parse::<i32>().unwrap();

            match guessed_number.cmp(&secret_number) {
                // x Ordering::Greater.then_with(f)
                // x if x < secret_number => {
                //     if (x + 10) < secret_number {
                //         println!("too small")
                //     } else {
                //         println!("close")
                //     }
                // }
                _ => {
                    println!("correct. You Win!");
                    correct_ans = true;
                    break 'game;
                }
            }
        }
    }
}
