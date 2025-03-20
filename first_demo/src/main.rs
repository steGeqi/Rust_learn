use std::io::stdin;
// std 标准库 io 模块 stdin 函数
// :: 用来访问库的公开访问的api

fn main() {
    let mut msg = String::new();
    println!("Enter your message:");
    stdin().read_line(&mut msg).unwrap();
    println!("Message: {}", msg);
}

// crate
// -- library crate (1)
// binary crate (n) 可执行文件



// rustc main.rs
// cargo run