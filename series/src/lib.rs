pub fn series(digits: &str, len: usize) -> Vec<String> {
    (0..digits.len()+1-len)
        .map(|s| digits[s..s+len].to_string())
        .collect()
}

