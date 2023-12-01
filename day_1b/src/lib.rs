use core::RunError;

static INPUT: &str = include_str!("../input.txt");

static NUMBERS: &[(&str, u8)] = &[
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

pub fn run() -> Result<String, RunError> {
    let lines = INPUT
        .split('\n')
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>();

    let mut total_value: u32 = 0;
    for line in lines {
        let calibration_value = extract_calibration_value(line)?;
        total_value += calibration_value;
    }

    Ok(total_value.to_string())
}

fn extract_calibration_value(string: &str) -> Result<u32, RunError> {
    let mut digits: Vec<u32> = Vec::new();

    for (i, char) in string.chars().enumerate() {
        if char.is_ascii_digit() {
            let digit = u32::try_from(char.to_digit(10).unwrap()).unwrap();
            digits.push(digit);
        } else {
            let substring = &string[i..];
            for (key, value) in NUMBERS {
                if substring.starts_with(key) {
                    digits.push(u32::from(value.to_owned()));
                }
            }
        }
    }

    if digits.len() == 1 {
        let first = digits.first().unwrap();
        Ok((first * 10) + first)
    } else if digits.len() > 1 {
        let first = digits.first().unwrap();
        let last = digits.last().unwrap();
        Ok((first * 10) + last)
    } else {
        Err(RunError {
            message: String::from("Failed to parse any digits!"),
        })
    }
}
