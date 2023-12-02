use core::RunError;

pub static TASKS: &[(&'static str, fn() -> Result<String, RunError>)] = &[
    ("day_1a", day_1a::run),
    ("day_1b", day_1b::run),
    ("day_2a", day_2a::run),
];
