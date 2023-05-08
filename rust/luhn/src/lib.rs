/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    code.chars()
        .try_rfold((0, 0), |(i, sum), c| match c {
            ' ' => Some((i, sum)),
            _ => c
                .to_digit(10)
                .map(|x| x * (i % 2 + 1))
                .map(|x| (i + 1, sum + x - 9 * (x > 9) as u32)),
        })
        .map_or(false, |(i, sum)| i > 1 && sum % 10 == 0)
}
