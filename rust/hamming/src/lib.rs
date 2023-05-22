/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    s1.len()
        .eq(&s2.len())
        .then_some(s1.bytes().zip(s2.bytes()).filter(|(a, b)| a != b).count())
}
