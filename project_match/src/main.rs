use std::io;
fn main() {
    println!("Enter a greeting:");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read a line");

    match name.trim() {
        "Good Bye" => println!("Sorry to see you go"),
        "Hello" => println!("Hi, nice to meet you!"),
        "How are you" => println!("I'm doing well!"),
        _ => println!("I can't find a greeting, good bye."),
    }
}
