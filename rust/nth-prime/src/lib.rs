pub fn nth(n: u32) -> u32 {
    let mut primes = vec![2];
    match n as usize {
        n if n == 0 => primes[n],
        n => (3..)
            .step_by(2)
            .filter(|&i| match primes.iter().all(|&p| i % p != 0) {
                true => primes.push(i).eq(&()),
                _ => false,
            })
            .nth(n - 1)
            .unwrap(),
    }
}
