pub fn parse_values(string: &str) -> Vec<i32> {
    string.split(' ').map(|s| s.parse().unwrap()).collect()
}
