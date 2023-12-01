use aoc_23::TASKS;
use core::RunError;
use std::io;

fn main() {
    println!("Enter the task to run (e.g. 'day_1a') or 'all':");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input!");

    let trimmed = input.trim();

    if trimmed == "all" {
        TASKS.iter().for_each(|t| run(t.0, t.1));
    } else {
        let task = TASKS
            .iter()
            .find(|t| t.0 == trimmed)
            .expect("Task not found.");
        run(task.0, task.1);
    }
}

fn run(name: &str, task: fn() -> Result<String, RunError>) {
    println!("Task: {}", name);
    match task() {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("Error: {}", error.message),
    }
}
