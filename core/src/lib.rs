pub struct RunError {
    pub message: String,
}

pub fn parse_lines(input: &str) -> Vec<&str> {
    input
        .split('\n')
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
}
