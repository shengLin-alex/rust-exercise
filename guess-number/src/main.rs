extern crate rand;

mod guess;

use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");

    // 產生亂術
    let secret_num = rand::thread_rng().gen_range(1, 101);

    // 進入迴圈
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // 讀取輸入數字
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        // 轉換為 u32，若結果為 Err 則繼續重新輸入
        let guess_num: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let guess = guess::Guess::new(guess_num);

        println!("your guess: {:?}", guess);

        match guess.value().cmp(&secret_num) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
}
