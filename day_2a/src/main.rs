pub fn main() {
    match day_2a::run() {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("Error: {}", error.message),
    }
}
