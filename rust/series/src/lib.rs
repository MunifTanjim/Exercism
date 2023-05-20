pub fn series(digits: &str, len: usize) -> Vec<String> {
    match len {
        0 => (0..digits.len() + 1).map(|_| "".into()).collect(),
        _ => digits
            .as_bytes()
            .windows(len)
            .map(|chunk| String::from_utf8_lossy(chunk).into())
            .collect(),
    }
}
