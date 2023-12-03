pub fn main() {
    match day_3b::run() {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("Error: {}", error.message),
    }
}
