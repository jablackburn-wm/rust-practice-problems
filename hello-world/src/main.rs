use std::io;

fn main() {
    println!("Hello, world!");

    let mut name: String = String::new();

    println!("What's your name?");
    io::stdin().read_line(&mut name).expect("failed to read input");
    
    println!("How do you do, {}", name);
}
