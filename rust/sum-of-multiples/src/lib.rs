use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors
        .iter()
        .filter(|&&factor| factor > 0)
        .fold(HashSet::new(), |mut set, &factor| {
            let mut mul = factor;
            loop {
                if mul >= limit {
                    break set;
                }
                set.insert(mul);
                mul += factor;
            }
        })
        .iter()
        .sum()
}
