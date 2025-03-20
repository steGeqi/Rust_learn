//use std::io;
use rand::Rng;
use std::cmp::Ordering;
use rpassword::read_password;  // 秘文输入
// rand 库 cargo add rand@0.85
// cargo update rand

fn main() {
    println!("Guess the number!");
    // 生成器
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("Please input your guess.");
    loop {

        // mut 可以重新赋值
        let guess = read_password().expect("Failed to read input");
        println!("You guessed: {}", guess);

        let guess: u32 =  match guess.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break
            },
        }
    }
}