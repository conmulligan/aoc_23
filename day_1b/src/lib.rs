static NUMBERS: &[(&str, i8)] = &[
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

pub fn run() {
    println!("Day 1b");
    println!("======");

    let content = include_str!("../input.txt");

    let lines = content
        .split('\n')
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>();

    let mut total_value: i32 = 0;
    for line in lines {
        let calibration_value = extract_calibration_value(line);
        total_value += i32::from(calibration_value);
    }

    println!("Result: {}", total_value);
}

fn extract_calibration_value(string: &str) -> i8 {
    let mut digits: Vec<i8> = Vec::new();

    for (i, char) in string.chars().enumerate() {
        if char.is_ascii_digit() {
            let digit = i8::try_from(char.to_digit(10).unwrap()).unwrap();
            digits.push(digit);
        } else {
            let substring = &string[i..];
            for (key, value) in NUMBERS {
                if substring.starts_with(key) {
                    digits.push(value.to_owned());
                }
            }
        }
    }

    if digits.len() == 1 {
        let first = digits.first().unwrap();
        (first * 10) + first
    } else if digits.len() > 1 {
        let first = digits.first().unwrap();
        let last = digits.last().unwrap();
        (first * 10) + last
    } else {
        panic!("Failed to parse any digits!");
    }
}
