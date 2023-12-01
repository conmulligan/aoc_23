pub fn days() -> &'static [(fn(), &'static str)] {
    &[(day_1a::run, "day_1a"), (day_1b::run, "day_1b")]
}
