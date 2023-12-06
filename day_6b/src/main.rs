pub fn main() {
    match day_6b::run() {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("Error: {}", error.message),
    }
}
