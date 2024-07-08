pub fn generate_string(length: usize) -> String {
    std::iter::repeat_with(fastrand::alphanumeric)
        .take(length)
        .collect()
}
