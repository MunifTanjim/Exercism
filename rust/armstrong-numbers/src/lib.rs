pub fn is_armstrong_number(num: u32) -> bool {
    let pow = (num as f32).log10() as u32 + 1;
    num.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap().pow(pow))
        .try_fold(0 as u32, |sum, n| sum.checked_add(n))
        .eq(&Some(num))
}
