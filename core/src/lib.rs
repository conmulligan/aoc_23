pub struct RunError {
    pub message: String,
}

pub fn parse_lines(input: &str) -> Vec<&str> {
    input
        .split('\n')
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}
