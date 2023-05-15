pub fn factors(mut n: u64) -> Vec<u64> {
    let mut factors = vec![];
    let mut fs = (3..n).step_by(2);
    let mut f = 2;
    loop {
        if n < 2 {
            break factors;
        } else if n % f == 0 {
            factors.push(f);
            n = n / f;
        } else {
            f = fs.next().unwrap();
        }
    }
}
