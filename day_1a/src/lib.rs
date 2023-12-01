pub fn run() {
    println!("Day 1a");
    println!("======");

    let content = include_str!("../input.txt");

    let lines = content
        .split('\n')
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>();

    let mut total_value: i32 = 0;
    for line in lines {
        let calibration_value = extract_calibration_value(line);
        total_value = total_value + i32::from(calibration_value);
    }

    println!("Result: {}", total_value);
}

fn extract_calibration_value(string: &str) -> i8 {
    let digits: Vec<char> = string.chars().filter(|c| c.is_ascii_digit()).collect();

    let joined: String;
    if digits.len() == 1 {
        joined = String::from_iter([digits.first().unwrap(), digits.first().unwrap()]);
    } else if digits.len() > 1 {
        joined = String::from_iter([digits.first().unwrap(), digits.last().unwrap()])
    } else {
        panic!("Failed to parse any digits!");
    }

    joined.parse().unwrap()
}
