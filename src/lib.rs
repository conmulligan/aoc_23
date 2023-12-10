use core::RunError;

pub static TASKS: &[(&'static str, fn() -> Result<String, RunError>)] = &[
    ("day_1a", day_1a::run),
    ("day_1b", day_1b::run),
    ("day_2a", day_2a::run),
    ("day_2b", day_2b::run),
    ("day_3a", day_3a::run),
    ("day_3b", day_3b::run),
    ("day_4a", day_4a::run),
    ("day_4b", day_4b::run),
    ("day_6a", day_6a::run),
    ("day_6b", day_6b::run),
    ("day_7a", day_7a::run),
    ("day_7b", day_7b::run),
    ("day_8a", day_8a::run),
    ("day_8b", day_8b::run),
    ("day_9a", day_9a::run),
    ("day_9b", day_9b::run),
];
