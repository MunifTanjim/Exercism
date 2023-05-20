pub fn collatz(n: u64) -> Option<u64> {
    match n {
        0 => return None,
        1 => return Some(0),
        _ if n % 2 == 0 => collatz(n / 2).map(|s| s + 1),
        _ => collatz(n.checked_mul(3)?.checked_add(1)?).map(|s| s + 1),
    }
}
