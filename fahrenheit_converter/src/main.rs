use std::io::{self, Write}; // "Write" needed to flush buffer contents (ln 6)

fn main() {
    println!("convert fahrenheit to celcius");
    print!("fahrenheit: "); // using print! macro to keep input on one line 
    io::stdout().flush().unwrap(); // flush buffer contents to push out print! immediately 

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("failed to read input");
    let fahrenheit: f32 = input.trim().parse().unwrap();

    let celcius: f32 = (fahrenheit - 32.0) * 0.5555; 

    println!("{fahrenheit} degrees fahrenheit in celcius: {celcius}");
}



