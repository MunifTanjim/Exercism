use std::{collections::HashMap, thread};

fn count(chunk: String) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    chunk
        .chars()
        .filter(|c| c.is_alphabetic())
        .for_each(|c| *map.entry(c.to_ascii_lowercase()).or_insert(0) += 1);
    map
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let input_len = input.len();
    let chunk_len = input_len / worker_count + 1;

    if chunk_len < 7 {
        return count(input.concat());
    }

    let mut handles = vec![];
    for lines in input.chunks(chunk_len) {
        let chunk = lines.concat();
        handles.push(thread::spawn(move || count(chunk)));
    }

    let mut map = HashMap::new();

    for handle in handles {
        handle
            .join()
            .unwrap()
            .iter()
            .for_each(|(&c, &n)| *map.entry(c).or_insert(0) += n)
    }

    map
}
