pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<u8> = vec![];
    for &ch in string.as_bytes() {
        match ch {
            b'(' => stack.push(b')'),
            b'{' => stack.push(b'}'),
            b'[' => stack.push(b']'),
            b')' | b'}' | b']' if Some(ch) != stack.pop() => return false,
            _ => {}
        }
    }
    stack.is_empty()
}
