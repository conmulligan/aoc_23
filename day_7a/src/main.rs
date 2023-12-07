pub fn main() {
    match day_7a::run() {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("Error: {}", error.message),
    }
}
