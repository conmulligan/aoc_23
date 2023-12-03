use core::RunError;

static INPUT: &str = include_str!("../input.txt");

pub fn run() -> Result<String, RunError> {
    let lines = core::parse_lines(INPUT);

    let mut total_value: u32 = 0;
    for line in lines {
        let calibration_value = extract_calibration_value(line)?;
        total_value += calibration_value;
    }

    Ok(total_value.to_string())
}

fn extract_calibration_value(string: &str) -> Result<u32, RunError> {
    let digits: Vec<u32> = string
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    if digits.len() == 1 {
        let first = digits.first().unwrap();
        Ok((first * 10) + first)
    } else if digits.len() > 1 {
        let first = digits.first().unwrap();
        let last = digits.last().unwrap();
        Ok((first * 10) + last)
    } else {
        Err(RunError {
            message: "Failed to parse any digits!".to_string(),
        })
    }
}
